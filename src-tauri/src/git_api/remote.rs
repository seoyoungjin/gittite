//!

// $ git remote -v
// origin    https://github.com/seoyoungjin/gittite.git (fetch)
// origin    https://github.com/seoyoungjin/gittite.git (push)

// TODO 
// how to get URL?
// fetch? push?

#![deny(warnings)]
#![allow(dead_code)]

use anyhow::{anyhow, Result};
use git2::Repository;
use super::repository::{repo_open, RepoPath};

/// origin
pub const DEFAULT_REMOTE_NAME: &str = "origin";

///
pub fn get_remotes(repo_path: &RepoPath) -> Result<Vec<String>> {
    let repo = repo_open(repo_path)?;
    let remotes = repo.remotes()?;
    let remotes: Vec<String> =
        remotes.iter().flatten().map(String::from).collect();

    Ok(remotes)
}

/// tries to find origin or the only remote that is defined if any
/// in case of multiple remotes and none named *origin* we fail
pub fn get_default_remote(repo_path: &RepoPath) -> Result<String> {
    let repo = repo_open(repo_path)?;
    get_default_remote_in_repo(&repo)
}

/// see `get_default_remote`
fn get_default_remote_in_repo(repo: &Repository,) -> Result<String> {
    let remotes = repo.remotes()?;

    // if `origin` exists return that
    let found_origin = remotes.iter().any(|r| {
        r.map(|r| r == DEFAULT_REMOTE_NAME).unwrap_or_default()
    });
    if found_origin {
        return Ok(DEFAULT_REMOTE_NAME.into());
    }

    //if only one remote exists pick that
    if remotes.len() == 1 {
        let first_remote = remotes
            .iter()
            .next()
            .flatten()
            .map(String::from)
            .ok_or_else(|| {
                anyhow!("no remote found")
            })?;

        return Ok(first_remote);
    }

    // TODO
    //inconclusive
    Err(anyhow!("no default remote found"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_api::tests::{
        debug_cmd_print, repo_clone, repo_init,
    };

    #[test]
    fn test_smoke() {
        let (remote_dir, _remote) = repo_init().unwrap();
        let remote_path = remote_dir.path().to_str().unwrap();
        let (repo_dir, _repo) = repo_clone(remote_path).unwrap();
        let repo_path: &RepoPath = &repo_dir
            .into_path()
            .as_os_str()
            .to_str()
            .unwrap()
            .into();

        let remotes = get_remotes(repo_path).unwrap();
        assert_eq!(remotes, vec![String::from("origin")]);
    }

    #[test]
    fn test_default_remote() {
        let (remote_dir, _remote) = repo_init().unwrap();
        let remote_path = remote_dir.path().to_str().unwrap();
        let (repo_dir, _repo) = repo_clone(remote_path).unwrap();
        let repo_path: &RepoPath = &repo_dir
            .into_path()
            .as_os_str()
            .to_str()
            .unwrap()
            .into();

        debug_cmd_print(
            repo_path,
            &format!("git remote add second {}", remote_path)[..],
        );

        let remotes = get_remotes(repo_path).unwrap();
        assert_eq!(
            remotes,
            vec![String::from("origin"), String::from("second")]
        );

        let repo = repo_open(repo_path).unwrap();
        let first = get_default_remote_in_repo(&repo).unwrap();
        assert_eq!(first, String::from("origin"));
    }

    #[test]
    fn test_default_remote_out_of_order() {
        let (remote_dir, _remote) = repo_init().unwrap();
        let remote_path = remote_dir.path().to_str().unwrap();
        let (repo_dir, _repo) = repo_clone(remote_path).unwrap();
        let repo_path: &RepoPath = &repo_dir
            .into_path()
            .as_os_str()
            .to_str()
            .unwrap()
            .into();

        debug_cmd_print(repo_path, "git remote rename origin alternate");
        debug_cmd_print(
            repo_path,
            &format!("git remote add origin {}", remote_path)[..],
        );

        //NOTE: aparently remotes are not chronolically sorted but alphabetically
        let remotes = get_remotes(repo_path).unwrap();
        assert_eq!(
            remotes,
            vec![String::from("alternate"), String::from("origin")]
        );

        let repo = repo_open(repo_path).unwrap();
        let first = get_default_remote_in_repo(&repo).unwrap();
        assert_eq!(first, String::from("origin"));
    }

    #[test]
    fn test_default_remote_inconclusive() {
        let (remote_dir, _remote) = repo_init().unwrap();
        let remote_path = remote_dir.path().to_str().unwrap();
        let (repo_dir, _repo) = repo_clone(remote_path).unwrap();
        let repo_path: &RepoPath = &repo_dir
            .into_path()
            .as_os_str()
            .to_str()
            .unwrap()
            .into();

        debug_cmd_print(repo_path, "git remote rename origin alternate");
        debug_cmd_print(
            repo_path,
            &format!("git remote add someremote {}", remote_path)[..],
        );

        let remotes = get_remotes(repo_path).unwrap();
        assert_eq!(
            remotes,
            vec![
                String::from("alternate"),
                String::from("someremote")
            ]
        );

        let repo = repo_open(repo_path).unwrap();
        let res = get_default_remote_in_repo(&repo);
        assert_eq!(res.is_err(), true);
    }
}
