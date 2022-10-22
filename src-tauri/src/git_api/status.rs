// #![deny(warnings)]

use crate::throw;
use anyhow::{anyhow, Result};
use git2::{Status, StatusOptions, StatusShow};
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::open::real_open;
use crate::app_data::AppDataState;

// https://git-scm.com/docs/git-status

// TOTO
// - untracked file
// - branch name
// - submodule

/// the result from 'git status'
#[derive(Serialize, Deserialize)]
pub struct StatusResult {
    /// current branch
    pub branch: String,
    /// current upstream branch
    // pub upstream_branch: String,
    /// tip commit of the current branch
    // pub tip: String,
    /// how many commits ahead and behind
    // pub branch_ahead: i32,
    // pub branch_behind: i32,
    /// working directory status
    pub working_dir: Vec<StatusItem>,
}

/// StatusItemType
#[derive(Serialize, Deserialize)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum StatusItemType {
    New,
    Modified,
    Deleted,
    Renamed,
    Typechange,
    Conflicted,
}

impl From<Status> for StatusItemType {
    fn from(s: Status) -> Self {
        if s.is_index_new() || s.is_wt_new() {
            Self::New
        } else if s.is_index_deleted() || s.is_wt_deleted() {
            Self::Deleted
        } else if s.is_index_renamed() || s.is_wt_renamed() {
            Self::Renamed
        } else if s.is_index_typechange() || s.is_wt_typechange() {
            Self::Typechange
        } else if s.is_conflicted() {
            Self::Conflicted
        } else {
            Self::Modified
        }
    }
}

/// StatusItem
#[derive(Serialize, Deserialize)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct StatusItem {
    pub path: String,
    pub status: StatusItemType,
}

fn run(repo: &Repository, opts: &mut StatusOptions) -> Result<StatusResult> {
    let statuses = repo.statuses(Some(opts))?;

    // branch name
    let branch = super::get_branch_name_repo(repo)?;

    // directory status
    let mut wds = Vec::with_capacity(statuses.len());
    for e in statuses.iter() {
        let status: Status = e.status();

        let path = match e.head_to_index() {
            Some(diff) => diff
                .new_file()
                .path()
                .and_then(Path::to_str)
                .map(String::from)
                .ok_or_else(|| anyhow!("failed to get path to diff's new file."))?,
            None => e
                .path()
                .map(String::from)
                .ok_or_else(|| anyhow!("failed to get the path to indexed file."))?,
        };

        wds.push(StatusItem {
            path,
            status: StatusItemType::from(status),
        });
    }

    wds.sort_by(|a, b| Path::new(a.path.as_str()).cmp(Path::new(b.path.as_str())));

    let response = StatusResult {
        branch: branch,
        working_dir: wds,
    };
    Ok(response)
}

#[tauri::command]
pub fn get_status(
    status_type: String,
    app_data: AppDataState<'_>
) -> Result<StatusResult, String> {
    let mut app_data = app_data.0.lock().unwrap();

    if app_data.repo.is_none() {
        real_open(&mut app_data)?;
    }
    let repo = app_data.repo.as_ref().unwrap();
    if repo.is_bare() {
        throw!("cannot report status on bare repository");
    }

    let status_show = match status_type.as_str() {
        "stage" => StatusShow::Index,
        "workdir" => StatusShow::Workdir,
        _ => StatusShow::IndexAndWorkdir,
    };

    let mut opts = StatusOptions::default();
    opts.show(status_show);
    opts.update_index(true);
    opts.renames_head_to_index(true);
    opts.include_ignored(false);
    opts.include_untracked(true);
    opts.recurse_untracked_dirs(true);
    opts.exclude_submodules(true);

    match run(repo, &mut opts) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("error: {}", e)),
    }
}
