//! git_api various methods

use anyhow::{anyhow, Result};
use super::CommitId;
use super::repository::{repo_open, RepoPath};
use git2::{Error, ErrorClass, ErrorCode, Repository};
use std::{fs::File, io::Write, path::{Path, PathBuf}};

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

//
pub(crate) fn work_dir(repo: &Repository) -> Result<&Path, std::io::Error> {
    repo.workdir().ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))
}

/// write a file in repo
pub(crate) fn repo_write_file(
    repo: &Repository,
    file: &str,
    content: &str,
) -> Result<()> {
    let dir = work_dir(repo)?.join(file);
    let file_path = dir.to_str().ok_or_else(|| {anyhow!("invalid file path")})?;
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
pub(crate) fn repo_read_file(
    repo: &Repository,
    file: &str,
) -> Result<String> {
    use std::io::Read;

    let dir = work_dir(repo)?.join(file);
    let file_path = dir.to_str().ok_or_else(|| {anyhow!("invalid file path")})?;

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(String::from_utf8(buffer)?)
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

