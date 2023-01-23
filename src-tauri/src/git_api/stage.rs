/*
 * libgit2 "add" example - shows how to modify the index
 *
 * Written by the libgit2 contributors
 *
 * To the extent possible under law, the author(s) have dedicated all copyright
 * and related and neighboring rights to this software to the public domain
 * worldwide. This software is distributed without any warranty.
 *
 * You should have received a copy of the CC0 Public Domain Dedication along
 * with this software. If not, see
 * <http://creativecommons.org/publicdomain/zero/1.0/>.
 */

// TODO
// #![deny(warnings)]
#![allow(trivial_casts)]

use super::error::Result;
use super::repository::{repo_open, RepoPath};
use std::ffi::OsString;
use std::path::Path;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(setting(AppSettings::NoBinaryName))]
struct Args {
    #[structopt(name = "spec")]
    arg_spec: Vec<String>,
    #[structopt(name = "update", short, long)]
    /// update tracked files
    flag_update: bool,
}

pub fn stage_add_all<I>(
    repo_path: &RepoPath,
    args: I,
) -> Result<()>
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    let args = Args::from_iter(args);
    let repo = repo_open(repo_path)?;
    let mut index = repo.index()?;

    if args.flag_update {
        index.update_all(args.arg_spec.iter(), None)?;
    } else {
        index.add_all(args.arg_spec.iter(), git2::IndexAddOption::DEFAULT, None)?;
    }
    index.write()?;
    Ok(())
}

/// add a file diff from workingdir to stage
pub fn stage_add_file(
    repo_path: &RepoPath,
    path: &Path,
) -> Result<()> {
    let repo = repo_open(repo_path)?;
    let mut index = repo.index()?;

    index.add_path(path)?;
    index.write()?;
    Ok(())
}

/// stage a removed file
pub fn stage_remove_file(
    repo_path: &RepoPath,
    path: &Path,
) -> Result<()> {
    let repo = repo_open(repo_path)?;
    let mut index = repo.index()?;

    let res = index.remove_path(path)?;
    log::trace!("remove_path : {:?}", res);
    index.write()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_api::tests::debug_cmd_print;
    use crate::git_api::tests::{get_statuses, repo_init};
    use crate::git_api::{commit::commit, repository::RepoPath, status::get_status};
    use git2::StatusShow;
    use std::{
        fs::{self, remove_file, File},
        io::Write,
        path::Path,
    };

    #[test]
    fn test_stage_add_smoke() {
        let file_path = Path::new("foo");
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        let res = stage_add_file(repo_path, file_path);
        assert_eq!(res.is_ok(), false);
    }

    #[test]
    fn test_stage_one_file() {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        let file_path = Path::new("foo");
        File::create(&root.join(file_path))
            .unwrap()
            .write_all(b"test file1 content")
            .unwrap();
        File::create(&root.join(Path::new("file2.txt")))
            .unwrap()
            .write_all(b"test file2 content")
            .unwrap();

        assert_eq!(get_statuses(repo_path), (2, 0));

        stage_add_file(repo_path, file_path).unwrap();
        assert_eq!(get_statuses(repo_path), (1, 1));
    }

    #[test]
    fn test_staging_folder() -> Result<()> {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        let status_count = |s: StatusShow| -> usize { get_status(repo_path, s).unwrap().len() };

        fs::create_dir_all(root.join("a/d"))?;
        File::create(root.join(Path::new("a/d/f1.txt")))?.write_all(b"foo")?;
        File::create(root.join(Path::new("a/d/f2.txt")))?.write_all(b"foo")?;
        File::create(root.join(Path::new("a/f3.txt")))?.write_all(b"foo")?;

        assert_eq!(status_count(StatusShow::Workdir), 3);

        stage_add_all(repo_path, vec!["a/d"]).unwrap();

        assert_eq!(status_count(StatusShow::Workdir), 1);
        assert_eq!(status_count(StatusShow::Index), 2);

        Ok(())
    }

    /* TODO
    #[test]
    fn test_not_staging_untracked_folder() -> Result<()> {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        fs::create_dir_all(root.join("a/d"))?;
        File::create(root.join(Path::new("a/d/f1.txt")))?.write_all(b"foo")?;
        File::create(root.join(Path::new("a/d/f2.txt")))?.write_all(b"foo")?;
        File::create(root.join(Path::new("f3.txt")))?.write_all(b"foo")?;

        assert_eq!(get_statuses(repo_path), (3, 0));

        repo.config()?.set_str("status.showUntrackedFiles", "no")?;
        assert_eq!(get_statuses(repo_path), (0, 0));

        stage_add_all(repo_path, vec!["*"]).unwrap();
        assert_eq!(get_statuses(repo_path), (0, 0));

        Ok(())
    }
    */

    #[test]
    fn test_staging_deleted_file() {
        let file_path = Path::new("file1.txt");
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        let status_count = |s: StatusShow| -> usize { get_status(repo_path, s).unwrap().len() };

        let full_path = &root.join(file_path);

        File::create(full_path)
            .unwrap()
            .write_all(b"test file1 content")
            .unwrap();

        stage_add_file(repo_path, file_path).unwrap();

        commit(repo_path, "commit msg").unwrap();

        // delete the file now
        assert_eq!(remove_file(full_path).is_ok(), true);

        // deleted file in diff now
        assert_eq!(status_count(StatusShow::Workdir), 1);

        stage_remove_file(repo_path, file_path).unwrap();

        assert_eq!(status_count(StatusShow::Workdir), 0);
        assert_eq!(status_count(StatusShow::Index), 1);
    }

    // see https://github.com/extrawurst/gitui/issues/108
    #[test]
    fn test_staging_sub_git_folder() -> Result<()> {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath = &root.as_os_str().to_str().unwrap().into();

        let status_count = |s: StatusShow| -> usize { get_status(repo_path, s).unwrap().len() };

        let sub = &root.join("sub");

        fs::create_dir_all(sub)?;

        debug_cmd_print(&sub.to_str().unwrap().into(), "git init subgit");

        File::create(sub.join("subgit/foo.txt"))
            .unwrap()
            .write_all(b"content")
            .unwrap();

        assert_eq!(status_count(StatusShow::Workdir), 1);

        //expect to fail
        assert!(stage_add_all(repo_path, vec!["sub"]).is_err());

        Ok(())
    }
}
