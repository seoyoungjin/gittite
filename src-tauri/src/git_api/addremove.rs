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

use super::repository::{repo_open, RepoPath};
use std::path::Path;
use std::ffi::OsString;
use structopt::StructOpt;
use structopt::clap::AppSettings;

#[derive(StructOpt)]
#[structopt(setting(AppSettings::NoBinaryName))]
struct Args {
    #[structopt(name = "spec")]
    arg_spec: Vec<String>,
    #[structopt(name = "update", short, long)]
    /// update tracked files
    flag_update: bool,
}

pub fn stage_add_all<I>(repo_path: &RepoPath, args: I) -> Result<(), git2::Error>
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone
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
pub fn stage_add_file(repo_path: &RepoPath, path: &Path) -> Result<(), git2::Error> {
    let repo = repo_open(repo_path)?;
    let mut index = repo.index()?;

    index.add_path(path)?;
    index.write()?;
    Ok(())
}

/// stage a removed file
pub fn stage_remove_file(repo_path: &RepoPath, path: &Path) -> Result<(), git2::Error> {
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
    use crate::git_api::repository::RepoPath;
    use crate::git_api::tests::{get_statuses, repo_init};
    use std::{fs::File, io::Write, path::Path};

    #[test]
    fn test_stage_add_smoke() {
        let file_path = Path::new("foo");
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath =
            &root.as_os_str().to_str().unwrap().into();

        let res = stage_add_file(repo_path, file_path);
        assert_eq!(res.is_ok(), false);
    }

    #[test]
    fn test_stage_one_file() {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path: &RepoPath =
            &root.as_os_str().to_str().unwrap().into();

        let file_path = Path::new("foo");
        File::create(&root.join(file_path)).unwrap()
            .write_all(b"test file1 content").unwrap();
        File::create(&root.join(Path::new("file2.txt"))).unwrap()
            .write_all(b"test file2 content").unwrap();

        assert_eq!(get_statuses(repo_path), (2, 0));

        stage_add_file(repo_path, file_path).unwrap();
        assert_eq!(get_statuses(repo_path), (1, 1));
    }
}
