/*
 * libgit2 "clone" example
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

//#![deny(warnings)]

use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{FetchOptions, Progress, RemoteCallbacks};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use structopt::clap::AppSettings;
use structopt::StructOpt;

// TODO
// - progress

#[derive(StructOpt)]
#[structopt(setting(AppSettings::NoBinaryName))]
pub struct Args {
    #[structopt(name = "url")]
    arg_url: String,
    #[structopt(name = "path")]
    arg_path: String,
}

struct State {
    progress: Option<Progress<'static>>,
    total: usize,
    current: usize,
    path: Option<PathBuf>,
}

fn print(state: &mut State) {
    println!("progress");
}

pub fn clone(args: &Args) -> Result<(), git2::Error> {
    let state = RefCell::new(State {
        progress: None,
        total: 0,
        current: 0,
        path: None,
    });
    let mut cb = RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        let mut state = state.borrow_mut();
        state.progress = Some(stats.to_owned());
        print(&mut *state);
        true
    });

    let mut co = CheckoutBuilder::new();
    co.progress(|path, cur, total| {
        let mut state = state.borrow_mut();
        state.path = path.map(|p| p.to_path_buf());
        state.current = cur;
        state.total = total;
        print(&mut *state);
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    RepoBuilder::new()
        .fetch_options(fo)
        .with_checkout(co)
        .clone(&args.arg_url, Path::new(&args.arg_path))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_api::tests::{repo_init, sandbox_config_files};
    use tempfile::TempDir;

    #[test]
    fn test_clone() {
        sandbox_config_files();

        let (r1_dir, _repo) = repo_init().unwrap();
        let r1_dir = r1_dir.path().to_str().unwrap();
        let td = TempDir::new().unwrap();
        let td_path = td.path().as_os_str().to_str().unwrap();

        let args = Args::from_iter(vec![r1_dir, td_path]);
        let res = clone(&args);
        assert_eq!(res.is_ok(), true);
    }
}
