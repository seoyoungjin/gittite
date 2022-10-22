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

pub fn stage_add(repo: &Repository, args: &Args) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    if args.flag_update {
        index.update_all(args.arg_spec.iter(), None)?;
    } else {
        index.add_all(args.arg_spec.iter(), git2::IndexAddOption::DEFAULT, None)?;
    }

    index.write()?;
    Ok(())
}

/*
pub fn stage_remove(repo: &Repository, args: &Args) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    index.remove_path(args.arg_spec.iter())?;
    index.write()?;
    Ok(())
}
*/

#[cfg(test)]
mod tests {
    fn add() {
        let args = Args::from_args();
        match run(&args) {
            Ok(()) => {}
            Err(e) => println!("error: {}", e),
        }
    }
}
