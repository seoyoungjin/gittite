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

// get/set value
#[derive(Serialize)]
pub enum Message {
    Bool(bool),
    Integer(i32),
    Str(String),
}

#[tauri::command]
pub fn get_param(
    name: String,
    app_data: AppDataState<'_>
) -> Result<Message, String> {
    let app_data = app_data.0.lock().unwrap();
    if name == "cwd" {
       return Ok(Message::Str(std::env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string()));
    }

    Ok(Message::Str("".to_string()))
}

#[tauri::command]
pub fn set_param(
    value: Value,
    app_data: AppDataState<'_>,
) -> Result<(), String> {
    let mut app_data = app_data.0.lock().unwrap();
    log::trace!("{:?}", value);

    Ok(())
}