//#![deny(warnings)]

use git2::Repository;
use std::path::{Path, PathBuf};

///
#[derive(Clone, Debug)]
pub struct RepoPath {
    gitdir: PathBuf,
    workdir: Option<PathBuf>,
}

impl RepoPath {
    ///
    pub fn gitpath(&self) -> &Path {
        self.gitdir.as_path()
    }

    ///
    pub fn workdir(&self) -> Option<&Path> {
        if let Some(wd) = &self.workdir {
            return Some(wd.as_path());
        }
        return None;
    }
}

impl From<&str> for RepoPath {
    fn from(p: &str) -> Self {
        Self {
            gitdir: PathBuf::from(p),
            workdir: None,
        }
    }
}

pub fn repo_open(repo_path: &RepoPath) -> Result<Repository, git2::Error> {
    let repo = Repository::discover(repo_path.gitpath())?;

    if let Some(workdir) = repo_path.workdir() {
        repo.set_workdir(workdir, false)?;
    }

    Ok(repo)
}

/* TODO
pub fn is_git_repo(path: &str) -> Result<String, git2::Error> {
    let repo = Repository::discover(Path.new(path))?;

    Ok(repo.workdir().)
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_api::tests::repo_init;
    use std::fs;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_repopath() {
        let mut repo_path = RepoPath::from("./foo/bar");
        let path = Path::new("./foo/bar");

        assert_eq!(repo_path.gitpath(), path);
        assert_eq!(repo_path.workdir.is_none(), true);

        repo_path.workdir = Some(PathBuf::from("./foo/bar"));
        assert_eq!(repo_path.workdir.is_none(), false);
        assert_eq!(repo_path.workdir(), Some(path));
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
