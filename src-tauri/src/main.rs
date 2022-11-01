#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::app_data::{ArcAppData, AppData};
use tauri::Manager;
use tauri::{Size, PhysicalSize};

mod app_data;
mod settings;
mod git_api;

#[macro_export]
macro_rules! throw {
  ($($arg:tt)*) => {{
    return Err(format!($($arg)*))
  }};
}

fn error_popup_main_thread(msg: String) {
  let builder = rfd::MessageDialog::new()
    .set_title("Error")
    .set_description(&msg)
    .set_buttons(rfd::MessageButtons::Ok)
    .set_level(rfd::MessageLevel::Info);
  builder.show();
}

fn main() {
  if cfg!(debug_assertions) {
    env_logger::init();
  }
  let settings = match settings::Settings::load() {
    Ok(v) => v,
    Err(e) => {
      error_popup_main_thread(e.to_string());
      settings::Settings::default()
    }
  };

  let app_data = AppData {
    settings: settings,
    repo_path: None,
  };

  let context = tauri::generate_context!();
  tauri::Builder::default()
    .manage(ArcAppData::new(app_data))
    .invoke_handler(tauri::generate_handler![
      app_data::get_settings,
      app_data::save_settings,
      git_api::clone,
      git_api::init,
      git_api::set_repo,
      git_api::commit,
      git_api::amend,
      git_api::commit_info,
      git_api::add,
      git_api::remove,
      git_api::reset_stage,
      git_api::get_commits,
      git_api::get_status,
      git_api::get_remotes,
    ])
    .setup(|app| {
      let win = app.get_window("main").unwrap();
      win.set_size(Size::Physical(PhysicalSize{width: 900, height: 800})).unwrap();
      Ok(())
    })
    .run(context)
    .expect("error while running tauri application");
}
