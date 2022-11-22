use crate::app_data::{AppData, AppDataState};
use std::sync::MutexGuard;

use crate::git_api::*;
use git2::StatusShow;
use serde_json::Value;
use std::path::Path;

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

#[tauri::command]
pub async fn clone(
    args: Vec<String>,
    app_data: AppDataState<'_>,
) -> Result<String, String> {
    log::trace!("clone args {:?}", args);
    let app_data = app_data.0.lock().unwrap();

    let tx_git = app_data.tx_git.clone();
    match clone::clone(&args, Some(tx_git)) {
        Ok(()) => Ok("Cloned".to_string()),
        Err(e) => return Err(e.to_string()),
    }
}

#[tauri::command]
pub fn set_repo(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<(), String> {
    log::trace!("setpo args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    let repo_path: RepoPath = args.as_str().into();
    log::trace!("repo_path: {:?}", repo_path);
    match repository::repo_open(&repo_path) {
        Ok(_repo) => {
            app_data.repo_path = Some(repo_path);
            Ok(())
        }
        Err(e) => return Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_status(
    status_type: String,
    app_data: AppDataState<'_>,
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
    app_data: AppDataState<'_>,
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
    app_data: AppDataState<'_>,
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
pub fn commit(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<CommitId, String> {
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
pub fn amend(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<CommitId, String> {
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
    app_data: AppDataState<'_>,
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
    app_data: AppDataState<'_>,
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
    path: String,
    stage: bool,
    app_data: AppDataState<'_>,
) -> Result<String, String> {
    log::trace!("get_diff:: path: {}, stage: {}", path, stage);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    // TODO
    let diff_opt = None;
    match diff::get_diff_string(repo_path, path.as_str(), stage, diff_opt) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_diff_commit(
    commit_id: String,
    path: Option<String>,
    app_data: AppDataState<'_>,
) -> Result<String, String> {
    log::trace!("get_diff_commit:: commit_id: {}", &commit_id[0..7]);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    // TODO error check
    let repo = repository::repo_open(repo_path).unwrap();
    let cid = CommitId::from_str(commit_id.as_str()).unwrap();
    // TODO error check
    let diff_opt = None;
    let diff = commit_files::get_commit_diff(repo_path, &repo, cid, path, diff_opt).unwrap();
    match utils::diff_to_string(&diff) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn add(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<bool, String> {
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
pub fn remove(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<bool, String> {
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
pub fn reset_stage(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<bool, String> {
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
pub fn get_branch_name(app_data: AppDataState<'_>) -> Result<String, String> {
    log::trace!("get_branch_name()");
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    match branch::get_branch_name(repo_path) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn create_branch(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<String, String> {
    log::trace!("create_branch() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    match branch::create_branch(repo_path, args.as_str()) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn delete_branch(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<(), String> {
    log::trace!("delete_branch() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    match branch::delete_branch(repo_path, args.as_str()) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn rename_branch(
    branch: String,
    name: String,
    app_data: AppDataState<'_>,
) -> Result<(), String> {
    log::trace!("rename_branch() with : {:?} {:?}", branch, name);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    match branch::rename_branch(repo_path, branch.as_str(), name.as_str()) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_branch_remote(
    branch: String,
    app_data: AppDataState<'_>,
) -> Result<Option<String>, String> {
    log::trace!("get_branch_remote() with : {:?}", branch);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    match branch::get_branch_remote(repo_path, branch.as_str()) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn branch_compare_upstream(
    branch: String,
    app_data: AppDataState<'_>,
) -> Result<BranchCompare, String> {
    log::trace!("branch_compare_upstream() with : {:?}", branch);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    match branch::branch_compare_upstream(repo_path, branch.as_str()) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_branches_info(
    local: bool,
    app_data: AppDataState<'_>,
) -> Result<Vec<BranchInfo>, String> {
    log::trace!("get_branches_info() with : {:?}", local);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    match branch::get_branches_info(repo_path, local) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn checkout_branch(
    branch_ref: String,
    app_data: AppDataState<'_>,
) -> Result<(), String> {
    log::trace!("checkout_branch() with : {:?}", branch_ref);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    match branch::checkout_branch(repo_path, branch_ref.as_str()) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn checkout_remote_branch(
    branch_ref: String,
    app_data: AppDataState<'_>,
) -> Result<(), String> {
    log::trace!("checkout_remote_branch() with : {:?}", branch_ref);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();

    let branches = match branch::get_branches_info(repo_path, false) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    for branch in branches {
        log::trace!("checkout_remote_branch() branch: {:?}", branch.name);
        if branch.name == branch_ref {
            match branch::checkout_remote_branch(repo_path, &branch) {
                Ok(()) => return Ok(()),
                Err(e) => return Err(e.to_string()),
            }
        }
    }
    return Err("can not find remote branch".to_string());
}

#[tauri::command]
pub fn get_remotes(app_data: AppDataState<'_>) -> Result<Vec<String>, String> {
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    match remotes::get_remotes(repo_path) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn tag(
    args: Vec<String>,
    app_data: AppDataState<'_>,
) -> Result<Value, String> {
    log::trace!("tag() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();
    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    match tag::tag(repo_path, &args) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn stash(
    args: Vec<String>,
    app_data: AppDataState<'_>,
) -> Result<Value, String> {
    log::trace!("stash() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();
    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    match stash::stash(repo_path, &args) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn blame(
    path: String,
    commit_id: Option<String>,
    app_data: AppDataState<'_>,
) -> Result<FileBlame, String> {
    log::trace!("blame() with : {:?}", path);
    let mut app_data = app_data.0.lock().unwrap();
    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    // TODO option?
    match blame::blame_file(repo_path, path.as_str(), None) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn test_progress(app_data: AppDataState<'_>) -> Result<(), String> {
    log::trace!("test_progress");
    let app_data = app_data.0.lock().unwrap();

    let tx_git = app_data.tx_git.clone();
    let handle = std::thread::spawn(move || {
        let millis = std::time::Duration::from_millis(500);
        let mut frames = 0;
        loop {
            tx_git
                .send(
                    ProgressNotification::Transfer {
                        objects: frames * 100,
                        total_objects: 1000,
                    }
                    .into(),
                )
                .expect("send progress error");
            frames += 1;
            if frames > 10 {
                break;
            }
            std::thread::sleep(millis)
        }
    });
    handle.join().unwrap();

    Ok(())
}
