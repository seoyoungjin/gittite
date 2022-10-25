//! Git API

use anyhow::{anyhow,Result};
use std::path::Path;
use git2::Repository;
use structopt::StructOpt;

use crate::throw;
use crate::app_data::AppDataState;

/// ditto
pub(crate) fn get_branch_name_repo(
    repo: &Repository,
) -> Result<String> {
    let iter = repo.branches(None)?;

    for b in iter {
        let b = b?;
        if b.0.is_head() {
            let name = b.0.name()?.unwrap_or("");
            return Ok(name.into());
        }
    }
    Err(anyhow!("git: no head found"))
}

// init, clone, open
pub mod init;
pub mod clone;
pub mod open;
use open::real_open;

pub mod addremove;
// mv rm restore

// pub mod diff;
pub mod revlog;
pub use self::revlog::*;
pub mod status;
pub use self::status::*;
// show grep

// branch commit merge rebase reset switch tag
// pub mod tag;
// pub mod blame;

// push
// pub mod fetch;
// pub mod pull;

// sig tree tag commit blob
// pub mod cat-file;

// remote head.oid, head.name (push, fetch)
// pub mod ls-remote;

// spec revspec
// pub mod rev-list;
// pub mod rev-parse;

#[tauri::command]
pub fn init(args: Vec<String>) -> Result<String, String> {
    println!("init args {:?}", args);
    let opt = init::Args::from_iter(args);
    match init::init(&opt) {
        Ok(_repo) => Ok("Initialized empty Git repository".to_string()),
        Err(e) => throw!("error: {}", e),
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

#[tauri::command]
pub fn clone(args: Vec<String>, window: tauri::Window) -> Result<String, String> {
    println!("clone args {:?}", args);
    let opt = clone::Args::from_iter(args);

    // TODO can use for progress?
    window.emit("clone-progress", Payload { message: "Tauri is awesome!".into() }).unwrap();
    let ok = match clone::clone(&opt) {
        Ok(()) => Ok("Cloned".to_string()),
        Err(e) => throw!("error: {}", e),
    };
    window.emit("clone-progress", Payload { message: "Clone done!".into() }).unwrap();
    ok
}

#[tauri::command]
pub fn open(app_data: AppDataState<'_>) -> Result<(), String> {
    let mut app_data = app_data.0.lock().unwrap();

    real_open(&mut app_data)?;
    Ok(())
}

#[tauri::command]
pub fn add(args: String, app_data: AppDataState<'_>) -> Result<bool, String> {
    log::trace!("add() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    if app_data.repo.is_none() {
        real_open(&mut app_data)?;
    }
    let repo = app_data.repo.as_ref().unwrap();
    let path = Path::new(&args);
    match addremove::stage_add_file(repo, path) {
        Ok(()) => Ok(true),
        Err(e) => throw!("error: {}", e),
    }
}

#[tauri::command]
pub fn reset(args: String, app_data: AppDataState<'_>) -> Result<bool, String> {
    log::trace!("reset() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    if app_data.repo.is_none() {
        real_open(&mut app_data)?;
    }
    let repo = app_data.repo.as_ref().unwrap();
    let path = Path::new(&args);
    match addremove::stage_reset_file(repo, path) {
        Ok(()) => Ok(true),
        Err(e) => throw!("error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use git2::Repository;
    use tempfile::TempDir;

    // init log
    fn init_log() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Trace)
            .try_init();
    }

    /// Calling `set_search_path` with an empty directory makes sure that there
    /// is no git config interfering with our tests (for example user-local
    /// `.gitconfig`).
    #[allow(unsafe_code)]
    pub fn sandbox_config_files() {
        use git2::{opts::set_search_path, ConfigLevel};
        use std::sync::Once;

        static INIT: Once = Once::new();

        // Adapted from https://github.com/rust-lang/cargo/pull/9035
        INIT.call_once(|| unsafe {
            let temp_dir = TempDir::new().unwrap();
            let path = temp_dir.path();

            set_search_path(ConfigLevel::System, path).unwrap();
            set_search_path(ConfigLevel::Global, path).unwrap();
            set_search_path(ConfigLevel::XDG, path).unwrap();
            set_search_path(ConfigLevel::ProgramData, path).unwrap();
        });
    }

    ///
    pub fn repo_init_empty() -> Result<(TempDir, Repository)> {
        init_log();
        sandbox_config_files();

        let td = TempDir::new()?;
        let repo = Repository::init(td.path())?;
        {
            let mut config = repo.config()?;
            config.set_str("user.name", "name")?;
            config.set_str("user.email", "email")?;
        }
        Ok((td, repo))
    }

    /// Same as `repo_init`, but the repo is a bare repo (--bare)
    pub fn repo_init_bare() -> Result<(TempDir, Repository)> {
        init_log();
        let tmp_repo_dir = TempDir::new()?;
        let bare_repo = Repository::init_bare(tmp_repo_dir.path())?;
        Ok((tmp_repo_dir, bare_repo))
    }
}
