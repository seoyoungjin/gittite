use crate::git_api::{RemoteProgress, RepoPath};
use crate::settings::Settings;

use serde::Serialize;
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

///  application data for tauri manage
pub type AppDataState<'a> = State<'a, ArcAppData>;
pub struct ArcAppData(pub Arc<Mutex<AppData>>);

impl ArcAppData {
    pub fn new(app_data: AppData) -> Self {
        Self(Arc::new(Mutex::new(app_data)))
    }
}

#[tauri::command]
pub fn get_settings(app_data: AppDataState<'_>) -> Result<Value, String> {
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
    let mut app_data = app_data.0.lock().unwrap();
    app_data.settings = value;
    app_data.save_settings()
}

#[tauri::command]
pub fn get_prop(
    key: &str,
    app_data: AppDataState<'_>
) -> Result<String, String> {
    log::trace!("get_prop({})", key);
    let app_data = app_data.0.lock().unwrap();

    let res = match key {
        "CWD" => {
            let cwd = std::env::current_dir().unwrap();
            String::from(cwd.to_string_lossy())
        },
        _ => return Err("invalid prop".into()),
    };
    Ok(res)
}

#[tauri::command]
pub fn set_prop(
    key: &str,
    val: &str,
    win: tauri::Window,
    app_data: AppDataState<'_>,
) -> Result<(), String> {
    log::trace!("set_prop({}, {})", key, val);
    let mut app_data = app_data.0.lock().unwrap();

    match key {
        "modal" => {
            let menu_handle = win.menu_handle();
            app_data.modal = if val == "true" { true } else { false }; 
            menu_handle.get_item("preference").set_enabled(!app_data.modal).unwrap();
            menu_handle.get_item("init").set_enabled(!app_data.modal).unwrap();
            menu_handle.get_item("add_local").set_enabled(!app_data.modal).unwrap();
            menu_handle.get_item("clone").set_enabled(!app_data.modal).unwrap();
        },
        _ => (),
    };
    Ok(())
}
