use crate::git_api::repository::{repo_open, RepoPath};
use crate::git_api::{Error, RemoteProgress, Result};
use crate::settings::Settings;

use filetime::FileTime;
use serde_json::Value;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use tauri::State;

pub struct AppData {
    pub settings: Settings,
    pub repo_path: Option<RepoPath>,
    pub tx_git: mpsc::Sender<RemoteProgress>,
    pub modal: bool,
}

impl AppData {
    pub fn settings_ref(&self) -> &Settings {
        &self.settings
    }

    pub fn save_settings(&mut self) -> Result<(), String> {
        match self.settings.save() {
            Ok(_v) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }

    pub fn repo_path_ref(&self) -> &RepoPath {
        self.repo_path.as_ref().unwrap()
    }
}

/// application data for tauri manage
pub type AppDataState<'a> = State<'a, ArcAppData>;
pub struct ArcAppData(pub Arc<Mutex<AppData>>);

impl ArcAppData {
    pub fn new(app_data: AppData) -> Self {
        Self(Arc::new(Mutex::new(app_data)))
    }
}

#[tauri::command]
pub fn get_settings(app_data: AppDataState<'_>) -> Result<Value, String> {
    log::trace!("get_settings()");
    let app_data = app_data.0.lock().unwrap();
    match serde_json::to_value(app_data.settings_ref()) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("Error serializing {}", e)),
    }
}

#[tauri::command]
pub fn save_settings(
    value: Settings,
    app_data: AppDataState<'_>,
) -> Result<(), String> {
    log::trace!("save_settings()");
    let mut app_data = app_data.0.lock().unwrap();
    app_data.settings = value;
    app_data.save_settings()
}

#[tauri::command]
pub fn get_prop(
    key: &str,
    app_data: AppDataState<'_>,
) -> Result<String> {
    log::trace!("get_prop({})", key);
    let app_data = app_data.0.lock().unwrap();

    let res = match key {
        "CWD" => {
            let cwd = std::env::current_dir().unwrap();
            String::from(cwd.to_string_lossy())
        }
        "LastFetchedTime" => {
            let fetch_head = repo_open(app_data.repo_path_ref())?
                .path()
                .join("FETCH_HEAD");
            let stat = std::fs::metadata(fetch_head)?;
            if stat.len() > 0 {
                let mtime = FileTime::from_last_modification_time(&stat);
                mtime.unix_seconds().to_string()
            } else {
                "".to_string()
            }
        }
        _ => return Err(Error::Generic("invalid prop".into())),
    };
    Ok(res)
}

#[tauri::command]
pub fn set_prop(
    key: &str,
    val: &str,
    win: tauri::Window,
    app_data: AppDataState<'_>,
) -> Result<()> {
    log::trace!("set_prop({}, {})", key, val);
    let mut app_data = app_data.0.lock().unwrap();
    let all_menu = vec![
        "preference",
        "select",
        "init",
        "add-local",
        "clone",
        "show-devtools",
        "repo-settings",
        "branch-create",
        "branch-rename",
        "branch-delete",
        "branch-reset",
        "branch-stash",
    ];

    match key {
        "modal" => {
            let menu_handle = win.menu_handle();
            app_data.modal = if val == "true" { true } else { false };
            for menu_name in all_menu.iter() {
                menu_handle
                    .get_item(menu_name)
                    .set_enabled(!app_data.modal)
                    .unwrap();
            }
        }
        _ => (),
    };
    Ok(())
}
