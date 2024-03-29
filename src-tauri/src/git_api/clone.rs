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

#![deny(warnings)]

use super::remotes::push::ProgressNotification;
use super::{ProgressPercent, RemoteProgress};
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{FetchOptions, Progress, RemoteCallbacks};
use std::cell::RefCell;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(setting(AppSettings::NoBinaryName))]
struct Args {
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

// prevent too many progress
// rust 1.63
static PREV_PERCENT: Mutex<u8> = Mutex::new(0);

fn transfer_progress(
    state: &mut State,
    sender: &Option<Sender<RemoteProgress>>,
) {
    if let Some(sender) = sender {
        let stats = state.progress.as_ref().unwrap();

        let progress =
            ProgressPercent::new(stats.received_objects(), stats.total_objects()).progress;
        if progress == *PREV_PERCENT.lock().unwrap() {
            return;
        }
        *PREV_PERCENT.lock().unwrap() = progress;

        log::debug!(
            "transfer: {}/{}",
            stats.received_objects(),
            stats.total_objects()
        );
        sender
            .send(
                ProgressNotification::Transfer {
                    objects: stats.received_objects(),
                    total_objects: stats.total_objects(),
                }
                .into(),
            )
            .expect("send progress error");
    }
}

pub fn clone<I>(
    args: I,
    sender: Option<Sender<RemoteProgress>>,
) -> Result<(), git2::Error>
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    let args = Args::from_iter(args);
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
        transfer_progress(&mut *state, &sender);
        true
    });

    let mut co = CheckoutBuilder::new();
    co.progress(|path, cur, total| {
        let mut state = state.borrow_mut();
        state.path = path.map(|p| p.to_path_buf());
        state.current = cur;
        state.total = total;
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

        let args = vec![r1_dir, td_path];
        let res = clone(&args, None);
        assert_eq!(res.is_ok(), true);
    }
}
