//! Git API

use anyhow::{anyhow,Result, Error};
use git2::{Branch, BranchType, Repository};

use structopt::StructOpt;
use crate::throw;
use crate::app_data::AppDataState;

/// ditto
pub(crate) fn get_branch_name_repo(
    repo: &Repository,
) -> Result<String> {
    let iter = repo.branches(None)?;

    for b in iter {
        let b = b?;
        if b.0.is_head() {
            let name = b.0.name()?.unwrap_or("");
            return Ok(name.into());
        }
    }
    Err(anyhow!("git: no head found"))
}

// init, clone, open
pub mod init;
pub mod clone;
pub mod open;
use open::real_open;

pub mod add;
// mv rm restore

// pub mod diff;
pub mod log;
pub use self::log::*;
pub mod status;
pub use self::status::*;
// show grep

// branch commit merge rebase reset switch tag
// pub mod tag;
// pub mod blame;

// push
// pub mod fetch;
// pub mod pull;

// sig tree tag commit blob
// pub mod cat-file;

// remote head.oid, head.name (push, fetch)
// pub mod ls-remote;

// spec revspec
// pub mod rev-list;
// pub mod rev-parse;

#[tauri::command]
pub fn init(args: Vec<String>) -> Result<String, String> {
    println!("init args {:?}", args);
    let opt = init::Args::from_iter(args);
    match init::run(&opt) {
        Ok(()) => Ok("Initialized empty Git repository".to_string()),
        Err(e) => throw!("error: {}", e),
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

#[tauri::command]
pub fn clone(args: Vec<String>, window: tauri::Window) -> Result<String, String> {
    println!("clone args {:?}", args);
    let opt = clone::Args::from_iter(args);

    // TODO can use for progress?
    window.emit("clone-progress", Payload { message: "Tauri is awesome!".into() }).unwrap();
    let ok = match clone::run(&opt) {
        Ok(()) => Ok("Cloned".to_string()),
        Err(e) => throw!("error: {}", e),
    };
    window.emit("clone-progress", Payload { message: "Clone done!".into() }).unwrap();
    ok
}

#[tauri::command]
pub fn open(app_data: AppDataState<'_>) -> Result<(), String> {
    let mut app_data = app_data.0.lock().unwrap();
    real_open(&mut app_data)
}

#[tauri::command]
pub fn add(args: Vec<String>, app_data: AppDataState<'_>) -> Result<bool, String> {
    let mut app_data = app_data.0.lock().unwrap();

    if app_data.repo.is_none() {
        real_open(&mut app_data)?;
    }
    let repo = app_data.repo.as_ref().unwrap();
    let opt = add::Args::from_iter(args);
    match add::stage_add(repo, &opt) {
        Ok(()) => Ok(true),
        Err(e) => throw!("error: {}", e),
    }
}
