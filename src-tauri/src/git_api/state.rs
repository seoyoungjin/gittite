use super::error::Result;
use super::repository::{repo_open, RepoPath};
use git2::RepositoryState;

///
#[derive(Debug, PartialEq, Eq)]
pub enum RepoState {
    ///
    Clean,
    ///
    Merge,
    ///
    Rebase,
    ///
    Revert,
    ///
    Other,
}

impl From<RepositoryState> for RepoState {
    fn from(state: RepositoryState) -> Self {
        match state {
            RepositoryState::Clean => Self::Clean,
            RepositoryState::Merge => Self::Merge,
            RepositoryState::Revert => Self::Revert,
            RepositoryState::RebaseMerge => Self::Rebase,
            _ => {
                log::warn!("state not supported yet: {:?}", state);
                Self::Other
            }
        }
    }
}

///
pub fn repo_state(repo_path: &RepoPath) -> Result<RepoState> {
    let repo = repo_open(repo_path)?;
    let state = repo.state();

    Ok(state.into())
}
