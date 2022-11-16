// #![deny(warnings)]

use super::repository::{repo_open, RepoPath};
use super::error::{Error, Result};
use git2::{Delta, Status, StatusOptions, StatusShow};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

// https://git-scm.com/docs/git-status

// TOOO
// - untracked file
// - submodule

/// StatusItemType
#[derive(Serialize, Deserialize)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum StatusItemType {
    Added,
    Modified,
    Deleted,
    Renamed,
    Typechange,
    Conflicted,
    Unchanged,
    Untracked,
    // Copied,
    // Innored,
    // UpdatedButUnmerged
}

impl From<Status> for StatusItemType {
    fn from(s: Status) -> Self {
        if s.is_index_new() {
            Self::Added
        } else if s.is_index_modified() {
            Self::Modified
        } else if s.is_index_deleted() {
            Self::Deleted
        } else if s.is_index_renamed() {
            Self::Renamed
        } else if s.is_index_typechange() {
            Self::Typechange
        } else if s.is_conflicted() {
            Self::Conflicted
        } else {
            if s.is_wt_new() {
                Self::Untracked
            } else {
                Self::Unchanged
            }
        }
    }
}

impl From<Delta> for StatusItemType {
    fn from(d: Delta) -> Self {
        match d {
            Delta::Added => Self::Added,
            Delta::Deleted => Self::Deleted,
            Delta::Renamed => Self::Renamed,
            Delta::Typechange => Self::Typechange,
            _ => Self::Modified,
        }
    }
}

/// WStatusItemType
#[derive(Serialize, Deserialize)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum WStatusItemType {
    New,
    Modified,
    Deleted,
    Copied,
    Renamed,
    Typechange,
    Untracked,
    Ignored,
}

impl From<Status> for WStatusItemType {
    fn from(s: Status) -> Self {
        if s.is_wt_new() {
            Self::New
        } else if s.is_wt_deleted() {
            Self::Deleted
        } else if s.is_wt_renamed() {
            Self::Renamed
        } else if s.is_wt_typechange() {
            Self::Typechange
        } else if s.is_ignored() {
            Self::Ignored
        } else {
            Self::Modified
        }
        // TODO copied, untracked
    }
}

/// StatusItem
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct StatusItem {
    pub path: String,
    pub stage: Option<StatusItemType>,
    pub wtree: Option<WStatusItemType>,
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
        let st: Status = e.status();

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

        match status_show {
            StatusShow::Index => {
                res.push(StatusItem {
                    path,
                    stage: Some(StatusItemType::from(st)),
                    wtree: None
                });
            },
            StatusShow::Workdir => {
                res.push(StatusItem {
                    path,
                    stage: None,
                    wtree: Some(WStatusItemType::from(st))
                });
            },
            _ => {
                res.push(StatusItem {
                    path,
                    stage: Some(StatusItemType::from(st)),
                    wtree: Some(WStatusItemType::from(st))
                });
            }
        }
    }

    res.sort_by(|a, b| {
        Path::new(a.path.as_str()).cmp(Path::new(b.path.as_str()))
    });

    Ok(res)
}
