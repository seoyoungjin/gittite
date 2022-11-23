use crate::app_data::{AppData, AppDataState};
use crate::git_api::*;
use git2::StatusShow;
use serde_json::Value;
use std::path::Path;
use std::sync::MutexGuard;

fn verify_repo_path(app_data: &mut MutexGuard<'_, AppData>) {
    if app_data.repo_path.is_none() {
        let repo_path = app_data.settings.repo.as_str().into();
        // log::trace!("repo_path: {:?}", repo_path);
        app_data.repo_path = Some(repo_path);
    }
}

#[tauri::command]
pub fn init(args: Vec<String>) -> Result<String> {
    log::trace!("init args {:?}", args);
    init::init(&args)?;
    Ok("Initialized empty Git repository".to_string())
}

#[tauri::command]
pub async fn clone(
    args: Vec<String>,
    app_data: AppDataState<'_>,
) -> Result<String> {
    log::trace!("clone args {:?}", args);
    let app_data = app_data.0.lock().unwrap();

    let tx_git = app_data.tx_git.clone();
    clone::clone(&args, Some(tx_git))?;
    Ok("Repository cloned".to_string())
}

#[tauri::command]
pub fn set_repository(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<()> {
    log::trace!("set_reposiroty args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    let repo_path: RepoPath = args.as_str().into();
    repository::repo_open(&repo_path)?;
    app_data.repo_path = Some(repo_path);
    Ok(())
}

#[tauri::command]
pub fn get_status(
    status_type: String,
    app_data: AppDataState<'_>,
) -> Result<Vec<StatusItem>> {
    log::trace!("get_status args {:?}", status_type);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let status_show = match status_type.as_str() {
        "stage" => StatusShow::Index,
        "workdir" => StatusShow::Workdir,
        _ => StatusShow::IndexAndWorkdir,
    };
    status::get_status(app_data.repo_path_ref(), status_show)
}

#[tauri::command]
pub fn get_commits(
    args: Vec<String>,
    app_data: AppDataState<'_>,
) -> Result<Vec<CommitData>> {
    log::trace!("get_commits:: args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    revlog::get_commits(app_data.repo_path_ref(), &args)
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
) -> Result<CommitId> {
    log::trace!("commit:: args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    commit::commit(repo_path, args.as_str())
}

#[tauri::command]
pub fn amend(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<CommitId> {
    log::trace!("amend:: args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let head_id = utils::get_head(&repo_path)?;
    commit::amend(repo_path, head_id, args.as_str())
}

#[tauri::command]
pub fn commit_info(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<CommitInfo> {
    log::trace!("commit_info:: args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let cid = match CommitId::from_str(args.as_str()) {
        Ok(cid) => cid,
        Err(e) => return Err(Error::Git(e)),
    };
    commit_info::get_commit_info(repo_path, cid)
}

#[tauri::command]
pub fn commit_files(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<Vec<StatusItem>> {
    log::trace!("commit_files:: args {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let cid = match CommitId::from_str(args.as_str()) {
        Ok(cid) => cid,
        Err(e) => return Err(Error::Git(e)),
    };
    commit_files::get_commit_files(repo_path, cid, None)
}

#[tauri::command]
pub fn get_diff(
    path: String,
    stage: bool,
    app_data: AppDataState<'_>,
) -> Result<String> {
    log::trace!("get_diff:: path: {}, stage: {}", path, stage);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    // TODO
    let diff_opt = None;
    diff::get_diff_string(repo_path, path.as_str(), stage, diff_opt)
}

#[tauri::command]
pub fn get_diff_commit(
    commit_id: String,
    path: Option<String>,
    app_data: AppDataState<'_>,
) -> Result<String> {
    log::trace!("get_diff_commit:: commit_id: {}", &commit_id[0..7]);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let repo = match repository::repo_open(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(Error::Git(e)),
    };
    let cid = match CommitId::from_str(commit_id.as_str()) {
        Ok(cid) => cid,
        Err(e) => return Err(Error::Git(e)),
    };
    // TODO
    let diff_opt = None;
    let diff = commit_files::get_commit_diff(repo_path, &repo, cid, path, diff_opt).unwrap();
    utils::diff_to_string(&diff)
}

#[tauri::command]
pub fn add(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<()> {
    log::trace!("add() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let path = Path::new(&args);

    addremove::stage_add_file(repo_path, path)
}

#[tauri::command]
pub fn remove(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<()> {
    log::trace!("remove() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let path = Path::new(&args);

    addremove::stage_remove_file(repo_path, path)
}

#[tauri::command]
pub fn reset_stage(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<()> {
    log::trace!("reset_stage() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();
    let path = args.as_str();

    reset::reset_stage(repo_path, path)
}

#[tauri::command]
pub fn get_branch_name(app_data: AppDataState<'_>) -> Result<String> {
    log::trace!("get_branch_name()");
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();

    branch::get_branch_name(repo_path)
}

#[tauri::command]
pub fn create_branch(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<String> {
    log::trace!("create_branch() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();

    branch::create_branch(repo_path, args.as_str())
}

#[tauri::command]
pub fn delete_branch(
    args: String,
    app_data: AppDataState<'_>,
) -> Result<()> {
    log::trace!("delete_branch() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();

    branch::delete_branch(repo_path, args.as_str())
}

#[tauri::command]
pub fn rename_branch(
    branch: String,
    name: String,
    app_data: AppDataState<'_>,
) -> Result<()> {
    log::trace!("rename_branch() with : {:?} {:?}", branch, name);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();

    branch::rename_branch(repo_path, branch.as_str(), name.as_str())
}

#[tauri::command]
pub fn get_branch_remote(
    branch: String,
    app_data: AppDataState<'_>,
) -> Result<Option<String>> {
    log::trace!("get_branch_remote() with : {:?}", branch);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();

    branch::get_branch_remote(repo_path, branch.as_str())
}

#[tauri::command]
pub fn branch_compare_upstream(
    branch: String,
    app_data: AppDataState<'_>,
) -> Result<BranchCompare> {
    log::trace!("branch_compare_upstream() with : {:?}", branch);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();

    branch::branch_compare_upstream(repo_path, branch.as_str())
}

#[tauri::command]
pub fn get_branches_info(
    local: bool,
    app_data: AppDataState<'_>,
) -> Result<Vec<BranchInfo>> {
    log::trace!("get_branches_info() with : {:?}", local);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();

    branch::get_branches_info(repo_path, local)
}

#[tauri::command]
pub fn checkout_branch(
    branch_ref: String,
    app_data: AppDataState<'_>,
) -> Result<()> {
    log::trace!("checkout_branch() with : {:?}", branch_ref);
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();

    branch::checkout_branch(repo_path, branch_ref.as_str())
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
pub fn get_remotes(app_data: AppDataState<'_>) -> Result<Vec<String>> {
    let mut app_data = app_data.0.lock().unwrap();

    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();

    remotes::get_remotes(repo_path)
}

#[tauri::command]
pub fn tag(
    args: Vec<String>,
    app_data: AppDataState<'_>,
) -> Result<Value> {
    log::trace!("tag() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();
    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();

    tag::tag(repo_path, &args)
}

#[tauri::command]
pub fn stash(
    args: Vec<String>,
    app_data: AppDataState<'_>,
) -> Result<Value> {
    log::trace!("stash() with : {:?}", args);
    let mut app_data = app_data.0.lock().unwrap();
    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();

    stash::stash(repo_path, &args)
}

#[tauri::command]
pub fn blame(
    path: String,
    commit_id: Option<String>,
    app_data: AppDataState<'_>,
) -> Result<FileBlame> {
    log::trace!("blame() with : {:?}", path);
    let mut app_data = app_data.0.lock().unwrap();
    verify_repo_path(&mut app_data);
    let repo_path = app_data.repo_path_ref();

    // TODO option?
    blame::blame_file(repo_path, path.as_str(), None)
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
