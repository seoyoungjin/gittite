// #![deny(warnings)]

use super::repository::{repo_open, RepoPath};
use super::error::{Error, Result};
use git2::{Delta, Status, StatusOptions, StatusShow};
use std::path::Path;
use serde::{Serialize, Deserialize};

// https://git-scm.com/docs/git-status

// TOOO
// - untracked file
// - branch name
// - submodule

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

impl From<Delta> for StatusItemType {
    fn from(d: Delta) -> Self {
        match d {
            Delta::Added => Self::New,
            Delta::Deleted => Self::Deleted,
            Delta::Renamed => Self::Renamed,
            Delta::Typechange => Self::Typechange,
            _ => Self::Modified,
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

// TODO show_untracked: Option<ShowUntrackedFilesConfig>,
/// gurantees sorting
pub fn get_status(
    repo_path: &RepoPath,
    status_show: StatusShow
) -> Result<Vec<StatusItem>> {
    let repo = repo_open(repo_path)?;
    if repo.is_bare() && !repo.is_worktree() {
        return Ok(Vec::new());
    }

    let mut opts = StatusOptions::default();
    opts.show(status_show);
    opts.update_index(true);
    opts.renames_head_to_index(true);
    opts.include_ignored(false);
    opts.include_untracked(true);
    opts.recurse_untracked_dirs(true);
    opts.exclude_submodules(true);

    let statuses = repo.statuses(Some(&mut opts))?;

    // directory status
    let mut res = Vec::with_capacity(statuses.len());
    for e in statuses.iter() {
        let status: Status = e.status();

        let path = match e.head_to_index() {
            Some(diff) => diff
                .new_file()
                .path()
                .and_then(Path::to_str)
                .map(String::from)
                .ok_or_else(|| {
                    Error::Generic(
                        "failed to get path to diff's new file.".to_string()
                    )
                })?,
            None => e.path().map(String::from).ok_or_else(|| {
                Error::Generic(
                    "failed to get the path to indexed file.".to_string()
                )
            })?,
        };

        res.push(StatusItem {
            path,
            status: StatusItemType::from(status),
        });
    }

    res.sort_by(|a, b| {
        Path::new(a.path.as_str()).cmp(Path::new(b.path.as_str()))
    });

    Ok(res)
}
