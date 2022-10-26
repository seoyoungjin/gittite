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
pub mod repository;
use repository::{real_open, RepoPath};

// add remove
// TODO mv
pub mod addremove;
pub mod reset;
// reset_stage
// TODO reset_workdir

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
pub mod remote;

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
pub fn remove(args: String, app_data: AppDataState<'_>) -> Result<bool, String> {
    log::trace!("remove() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    if app_data.repo.is_none() {
        real_open(&mut app_data)?;
    }
    let repo = app_data.repo.as_ref().unwrap();
    let path = Path::new(&args);
    match addremove::stage_remove_file(repo, path) {
        Ok(()) => Ok(true),
        Err(e) => throw!("error: {}", e),
    }
}

#[tauri::command]
pub fn reset_stage(args: String, app_data: AppDataState<'_>) -> Result<bool, String> {
    log::trace!("reset_stage() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    let git_dir = &app_data.settings.repo;
    let repo_path = RepoPath::from(git_dir.as_str());
    let path = args.as_str();
    match reset::reset_stage(&repo_path, path) {
        Ok(()) => Ok(true),
        Err(e) => throw!("error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        RepoPath
    };
    use anyhow::Result;
    use git2::Repository;
    use std::{path::Path, process::Command};
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

    ///
    pub fn repo_init() -> Result<(TempDir, Repository)> {
        init_log();
        sandbox_config_files();

        let td = TempDir::new()?;
        let repo = Repository::init(td.path())?;
        {
            let mut config = repo.config()?;
            config.set_str("user.name", "name")?;
            config.set_str("user.email", "email")?;

            let mut index = repo.index()?;
            let id = index.write_tree()?;

            let tree = repo.find_tree(id)?;
            let sig = repo.signature()?;
            repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                "initial",
                &tree,
                &[],
            )?;
        }
        Ok((td, repo))
    }

    ///
    pub fn repo_clone(p: &str) -> Result<(TempDir, Repository)> {
        sandbox_config_files();

        let td = TempDir::new()?;
        let td_path = td.path().as_os_str().to_str().unwrap();
        let repo = Repository::clone(p, td_path).unwrap();

        let mut config = repo.config()?;
        config.set_str("user.name", "name")?;
        config.set_str("user.email", "email")?;

        Ok((td, repo))
    }


	///
	pub fn debug_cmd_print(path: &RepoPath, cmd: &str) {
		let cmd = debug_cmd(path, cmd);
		eprintln!("\n----\n{}", cmd);
	}

	fn debug_cmd(path: &RepoPath, cmd: &str) -> String {
		let output = if cfg!(target_os = "windows") {
			Command::new("cmd")
				.args(&["/C", cmd])
				.current_dir(path.gitpath())
				.output()
				.unwrap()
		} else {
			Command::new("sh")
				.arg("-c")
				.arg(cmd)
				.current_dir(path.gitpath())
				.output()
				.unwrap()
		};

		let stdout = String::from_utf8_lossy(&output.stdout);
		let stderr = String::from_utf8_lossy(&output.stderr);
		format!(
			"{}{}",
			if stdout.is_empty() {
				String::new()
			} else {
				format!("out:\n{}", stdout)
			},
			if stderr.is_empty() {
				String::new()
			} else {
				format!("err:\n{}", stderr)
			}
		)
	}
}
