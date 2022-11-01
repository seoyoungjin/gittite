//! git_api various methods

use super::CommitId;
use git2::{Error, ErrorClass, ErrorCode, Repository};
use super::repository::{repo_open, RepoPath};

///
pub fn get_head(repo_path: &RepoPath) -> Result<CommitId, Error> {
    let repo = repo_open(repo_path)?;
    get_head_repo(&repo)
}

///
pub fn get_head_repo(repo: &Repository) -> Result<CommitId, Error> {
    let head = repo.head()?.target();
    head.map_or(
        Err(Error::new(
            ErrorCode::NotFound,
            ErrorClass::Reference,
            "head not found".to_string()
        )),
        |head_id| Ok(head_id.into()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_api::tests::{init_log, repo_init};

    #[test]
    fn test_head() -> Result<(), Error> {
        init_log();
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();
        assert_eq!(get_head(repo_path).is_ok(), true);

        let head_id = get_head(repo_path).unwrap();
        let head_id2: CommitId = repo.refname_to_id("HEAD").unwrap().into();
        log::trace!("HEAD : {:?}", head_id2);
        assert_eq!(head_id, head_id2);

        Ok(())
    }
}

