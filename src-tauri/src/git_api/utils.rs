//! git_api various methods

use super::error::{Error, Result};
use super::repository::{repo_open, RepoPath};
use super::CommitId;
use git2::{Diff, DiffFormat, Repository};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use std::{
    io::{BufWriter, Write},
    path::Path,
};

/// helper function to calculate the hash of an arbitrary type
/// that implements the `Hash` trait
pub fn hash<T: Hash + ?Sized>(v: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}

//
pub(crate) fn work_dir(repo: &Repository) -> Result<&Path, std::io::Error> {
    repo.workdir()
        .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))
}

///
pub fn get_head(repo_path: &RepoPath) -> Result<CommitId> {
    let repo = repo_open(repo_path)?;
    get_head_repo(&repo)
}

///
pub fn get_head_repo(repo: &Repository) -> Result<CommitId> {
    let head = repo.head()?.target();
    head.map_or(
        Err(Error::Git(git2::Error::new(
            git2::ErrorCode::NotFound,
            git2::ErrorClass::Reference,
            "head not found".to_string(),
        ))),
        |head_id| Ok(head_id.into()),
    )
}

pub(crate) fn bytes2string(bytes: &[u8]) -> Result<String> {
    Ok(String::from_utf8(bytes.to_vec())?)
}

/// diff to stream
pub fn diff_to_stream<'a>(diff: &'a Diff) -> Result<Vec<u8>> {
    let mut buf = BufWriter::new(Vec::new());
    diff.print(DiffFormat::Patch, |_delta, _hunk, line: git2::DiffLine| {
        match line.origin() {
            '+' | '-' | ' ' => write!(buf, "{}", line.origin()).unwrap(),
            _ => {}
        }
        if buf.write_all(line.content()).is_err() {
            false
        } else {
            true
        }
    })?;
    let bytes = buf.into_inner().unwrap();
    Ok(bytes)
}

#[cfg(test)]
use std::fs::File;

#[cfg(test)]
pub(crate) fn repo_read_file(
    repo: &Repository,
    file: &str,
) -> Result<String> {
    use std::io::Read;

    let dir = work_dir(repo)?.join(file);
    let file_path = dir
        .to_str()
        .ok_or_else(|| Error::Generic(String::from("invalid file path")))?;

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(String::from_utf8(buffer)?)
}

/// write a file in repo
#[cfg(test)]
pub(crate) fn repo_write_file(
    repo: &Repository,
    file: &str,
    content: &str,
) -> Result<()> {
    let dir = work_dir(repo)?.join(file);
    let file_path = dir
        .to_str()
        .ok_or_else(|| Error::Generic(String::from("invalid file path")))?;
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
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
