/*
 * libgit2 "init" example - shows how to initialize a new repo
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

#![deny(warnings)]

use git2::{Error, Repository, RepositoryInitMode, RepositoryInitOptions};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(name = "directory")]
    arg_directory: String,
    #[structopt(name = "bare", long)]
    /// initialize a new bare repository
    flag_bare: bool,
    #[structopt(name = "dir", long = "template")]
    /// use <dir> as an initialization template
    flag_template: Option<String>,
    #[structopt(name = "separate-git-dir", long)]
    /// use <dir> as the .git directory
    flag_separate_git_dir: Option<String>,
    #[structopt(name = "perms", long = "shared")]
    /// permissions to create the repository with
    flag_shared: Option<String>,
}

pub fn run(args: &Args) -> Result<(), Error> {
    let mut path = PathBuf::from(&args.arg_directory);
    let _repo = if !args.flag_bare
        && args.flag_template.is_none()
        && args.flag_shared.is_none()
        && args.flag_separate_git_dir.is_none()
    {
        Repository::init(&path)?
    } else {
        let mut opts = RepositoryInitOptions::new();
        opts.bare(args.flag_bare);
        if let Some(ref s) = args.flag_template {
            opts.template_path(Path::new(s));
        }

        // If you specified a separate git directory, then initialize
        // the repository at that path and use the second path as the
        // working directory of the repository (with a git-link file)
        if let Some(ref s) = args.flag_separate_git_dir {
            opts.workdir_path(&path);
            path = PathBuf::from(s);
        }

        if let Some(ref s) = args.flag_shared {
            opts.mode(parse_shared(s)?);
        }
        Repository::init_opts(&path, &opts)?
    };

    Ok(())
}

fn parse_shared(shared: &str) -> Result<RepositoryInitMode, Error> {
    match shared {
        "false" | "umask" => Ok(git2::RepositoryInitMode::SHARED_UMASK),
        "true" | "group" => Ok(git2::RepositoryInitMode::SHARED_GROUP),
        "all" | "world" => Ok(git2::RepositoryInitMode::SHARED_ALL),
        _ => {
            if shared.starts_with('0') {
                match u32::from_str_radix(&shared[1..], 8).ok() {
                    Some(n) => Ok(RepositoryInitMode::from_bits_truncate(n)),
                    None => Err(Error::from_str("invalid octal value for --shared")),
                }
            } else {
                Err(Error::from_str("unknown value for --shared"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    fn main() {
        let args = Args::from_args();
        match run(&args) {
            Ok(()) => {}
            Err(e) => println!("error: {}", e),
        }
    }
}
