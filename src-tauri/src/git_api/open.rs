#![deny(warnings)]

use crate::app_data::AppData;
use git2::Repository;
use std::sync::MutexGuard;

pub fn real_open(app_data: &mut MutexGuard<'_, AppData>) -> Result<(), String> {
    let git_dir = &app_data.settings.repo;
    let repo = match Repository::open(git_dir) {
        Ok(v) => v,
        Err(e) => return Err(format!("error: {}", e)),
    };
    app_data.repo = Some(repo);
    Ok(())
}
