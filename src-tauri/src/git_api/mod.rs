//! Git API

// TODO
// pub type Result<T> = std::result::Result<T, String>;

use crate::app_data::{AppData, AppDataState};
use std::sync::MutexGuard;

use anyhow::anyhow;
use std::path::Path;
use git2::StatusShow;

pub mod init;
pub mod clone;
pub mod repository;

mod commit;
mod commit_info;
mod commit_files;

// TODO mv
pub mod addremove;
// TODO reset_workdir
pub mod reset;

pub mod diff;
pub mod revlog;
pub mod rev_list;
pub mod status;
// show grep

// branch merge rebase reset switch
// pub mod tag;
pub mod stash;
// pub mod blame;

// push
// pub mod fetch;
// pub mod pull;

// sig tree blob
// pub mod cat-file;

pub mod remote;

// spec revspec
// pub mod rev-parse;
pub mod utils;

use repository::RepoPath;
use commit_info::{CommitId, CommitInfo};
use revlog::CommitData;
use status::{StatusItem, StatusItemType};
use diff::FileDiff;

fn verify_repo_path(app_data: &mut MutexGuard<'_, AppData>) {
    if app_data.repo_path.is_none() {
        let repo_path = app_data.settings.repo.as_str().into();
        log::trace!("repo_path: {:?}", repo_path);
        app_data.repo_path = Some(repo_path);
    }
}

#[tauri::command]
pub fn init(args: Vec<String>) -> Result<String, String> {
    log::trace!("init args {:?}", args);
    match init::init(&args) {
        Ok(_repo) => Ok("Initialized empty Git repository".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

#[tauri::command]
pub fn clone(args: Vec<String>, window: tauri::Window) -> Result<String, String> {
    log::trace!("clone args {:?}", args);

    // TODO can use for progress?
    window.emit("clone-progress", Payload { message: "Tauri is awesome!".into() }).unwrap();
    let ok = match clone::clone(&args) {
        Ok(()) => Ok("Cloned".to_string()),
        Err(e) => return Err(e.to_string()),
    };
    window.emit("clone-progress", Payload { message: "Clone done!".into() }).unwrap();
    ok
}

#[tauri::command]
pub fn set_repo(args: String, app_data: AppDataState<'_>) -> Result<(), String> {
    log::trace!("setpo args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    let repo_path: RepoPath = args.as_str().into();
    log::trace!("repo_path: {:?}", repo_path);
    match repository::repo_open(&repo_path) {
        Ok(repo) => {
            app_data.repo_path = Some(repo_path);
            Ok(())
        },
        Err(e) => return Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_status(
    status_type: String,
    app_data: AppDataState<'_>
) -> Result<Vec<StatusItem>, String> {
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let status_show = match status_type.as_str() {
        "stage" => StatusShow::Index,
        "workdir" => StatusShow::Workdir,
        _ => StatusShow::IndexAndWorkdir,
    };
    match status::get_status(app_data.repo_path_ref(), status_show) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_commits(
    args: Vec<String>,
    app_data: AppDataState<'_>
) -> Result<Vec<CommitData>, String> {
    log::trace!("get_commits:: args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    match revlog::get_commits(app_data.repo_path_ref(), &args) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn rev_list(
    args: Vec<String>,
    app_data: AppDataState<'_>
) -> Result<(), String> {
    log::trace!("rev_list:: args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    match rev_list::rev_list(app_data.repo_path_ref(), &args) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn commit(args: String, app_data: AppDataState<'_>) -> Result<CommitId, String> {
    log::trace!("commit:: args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    match commit::commit(repo_path, args.as_str()) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn amend(args: String, app_data: AppDataState<'_>) -> Result<CommitId, String> {
    log::trace!("amend:: args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let head_id = match utils::get_head(&repo_path) {
        Ok(cid) => cid,
        Err(e) => return Err(e.to_string()),
    };
    match commit::amend(repo_path, head_id, args.as_str()) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn commit_info(
    args: String,
    app_data: AppDataState<'_>
) -> Result<CommitInfo, String> {
    log::trace!("commit_info:: args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let cid = match CommitId::from_str(args.as_str()) {
        Ok(cid) => cid,
        Err(e) => return Err(e.to_string()),
    };
    match commit_info::get_commit_info(repo_path, cid) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn commit_files(
    args: String,
    app_data: AppDataState<'_>
) -> Result<Vec<StatusItem>, String> {
    log::trace!("commit_files:: args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let cid = match CommitId::from_str(args.as_str()) {
        Ok(cid) => cid,
        Err(e) => return Err(e.to_string()),
    };
    match commit_files::get_commit_files(repo_path, cid, None) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_diff(
    args: String,
    app_data: AppDataState<'_>
) -> Result<FileDiff, String> {
    log::trace!("commit_files:: args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let path = args.as_str();
    let stage = true;
    let diff_opt = None;
    match diff::get_diff(repo_path, &path, stage, diff_opt) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn add(args: String, app_data: AppDataState<'_>) -> Result<bool, String> {
    log::trace!("add() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let path = Path::new(&args);

    match addremove::stage_add_file(repo_path, path) {
        Ok(()) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn remove(args: String, app_data: AppDataState<'_>) -> Result<bool, String> {
    log::trace!("remove() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let path = Path::new(&args);

    match addremove::stage_remove_file(repo_path, path) {
        Ok(()) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn reset_stage(args: String, app_data: AppDataState<'_>) -> Result<bool, String> {
    log::trace!("reset_stage() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let path = args.as_str();
    match reset::reset_stage(repo_path, path) {
        Ok(()) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_remotes(app_data: AppDataState<'_>) -> Result<Vec<String>, String> {
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    match remote::get_remotes(repo_path) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::{CommitId, RepoPath};
    use super::status::get_status;
    use super::utils::{get_head_repo, repo_write_file};
    use super::{commit, addremove};

    use anyhow::Result;
    use git2::{Repository, StatusShow};
    use std::{path::Path, process::Command};
    use tempfile::TempDir;

    // init log
    pub fn init_log() {
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

    /// helper returning amount of files with changes in the (wd,stage)
    pub fn get_statuses(repo_path: &RepoPath) -> (usize, usize) {
        (
            get_status(repo_path, StatusShow::Workdir).unwrap().len(),
            get_status(repo_path, StatusShow::Index).unwrap().len()
        )
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

    /// write, stage and commit a file
    pub fn write_commit_file(
        repo: &Repository,
        file: &str,
        content: &str,
        commit_name: &str,
    ) -> CommitId {
        repo_write_file(repo, file, content).unwrap();

        addremove::stage_add_file(
            &repo.workdir().unwrap().to_str().unwrap().into(),
            Path::new(file),
        ).unwrap();

        commit::commit(
            &repo.workdir().unwrap().to_str().unwrap().into(),
            commit_name,
        ).unwrap()
    }
}
