use super::error::Result;
use super::remotes::get_default_remote_in_repo;
use git2::Repository;
use serde::Serialize;
use std::path::{Path, PathBuf};

///
#[derive(Clone, Debug)]
pub enum RepoPath {
    ///
    Path(PathBuf),
    ///
    Workdir {
        ///
        gitdir: PathBuf,
        ///
        workdir: PathBuf,
    },
}

impl RepoPath {
    pub fn gitpath(&self) -> &Path {
        match self {
            Self::Path(p) => p.as_path(),
            Self::Workdir { gitdir, .. } => gitdir.as_path(),
        }
    }

    pub fn workdir(&self) -> Option<&Path> {
        match self {
            Self::Path(_) => None,
            Self::Workdir { workdir, .. } => Some(workdir.as_path()),
        }
    }
}

impl From<&str> for RepoPath {
    fn from(p: &str) -> Self {
        Self::Path(PathBuf::from(p))
    }
}

pub fn repo_open(repo_path: &RepoPath) -> Result<Repository, git2::Error> {
    let repo = Repository::discover(repo_path.gitpath())?;

    if let Some(workdir) = repo_path.workdir() {
        repo.set_workdir(workdir, false)?;
    }

    Ok(repo)
}

/// repository information

#[derive(Serialize)]
pub struct RepoInfo {
    path: String,
    is_bare: bool,
    // is_shallow: bool,
    // is_empty: bool,
    // state
    // current branch

    // origin
    default_remote: String,
}

pub fn get_repo_info(repo_path: &RepoPath) -> Result<RepoInfo> {
    let repo = repo_open(repo_path)?;

    let workdir = repo.workdir();
    let path = if workdir.is_none() {
        repo_path.gitpath()
    } else {
        workdir.unwrap()
    };

    Ok(RepoInfo {
        path: path.to_string_lossy().into(),
        is_bare: repo.is_bare(),
        // is_shallow: repo.is_shallow(),
        default_remote: get_default_remote_in_repo(&repo)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_api::tests::repo_init;
    use std::fs;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_repopath() {
        let repo_path = RepoPath::from("./foo/bar");
        let gitdir = PathBuf::from("./foo/bar");

        assert_eq!(repo_path.gitpath(), gitdir);
        assert_eq!(repo_path.workdir().is_none(), true);

        let wd = PathBuf::from("./foo/bar");
        let repo_path = RepoPath::Workdir {
            gitdir,
            workdir: wd,
        };
        assert_eq!(repo_path.workdir(), Some(Path::new("./foo/bar")));
    }

    #[test]
    fn test_repo_open() {
        let (td, _repo) = repo_init().unwrap();
        let path = td.path().join("foot");
        fs::create_dir(&path).unwrap();
        let repo_path: RepoPath = path.as_os_str().to_str().unwrap().into();

        // subdirectory
        let repo = Repository::open(repo_path.gitpath());
        assert_eq!(repo.is_ok(), false);

        let repo = repo_open(&repo_path);
        assert_eq!(repo.is_ok(), true);
    }
}
