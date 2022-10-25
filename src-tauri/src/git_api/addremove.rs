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
// #![allow(trivial_casts)]

use git2::Repository;
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(name = "spec")]
    arg_spec: Vec<String>,
    #[structopt(name = "update", short, long)]
    /// update tracked files
    flag_update: bool,
}

pub fn stage_add_all(repo: &Repository, args: &Args) -> Result<(), git2::Error> {
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
pub fn stage_add_file(repo: &Repository, path: &Path) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    index.add_path(path)?;
    index.write()?;
    Ok(())
}

/// stage a removed file
pub fn stage_reset_file(repo: &Repository, path: &Path) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    index.remove_path(path)?;
    index.write()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_api::tests::repo_init_empty;

    #[test]
    fn test_stage_add_sooke() {
        let file_path = Path::new("foo");
        let (_td, repo) = repo_init_empty().unwrap();
        // let root = repo.path().parent().unwrap();
        // let repo_path = root.as_os_str().to_str().unwrap();
        // let args = Args::from_iter(vec![ "foo" ]);
        let res = stage_add_file(&repo, file_path);
        assert_eq!(res.is_ok(), false);
    }

    #[test]
    fn test_stage_remove() {
    }
}
