#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::app_data::{ArcAppData, AppData};
use crate::git_api::RemoteProgress;
use tauri::Manager;
use tauri::{Size, PhysicalSize};

mod app_data;
mod settings;
mod cmd;
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

    // progress message
    let (tx_git, rx_git) = std::sync::mpsc::channel::<RemoteProgress>();

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app_data::get_settings,
            app_data::save_settings,
            cmd::clone,
            cmd::init,
            cmd::set_repo,
            cmd::commit,
            cmd::amend,
            cmd::commit_info,
            cmd::commit_files,
            cmd::add,
            cmd::remove,
            cmd::reset_stage,
            cmd::get_commits,
            cmd::rev_list,
            cmd::get_status,
            cmd::get_remotes,
            cmd::get_diff,
            // cmd::get_diff_commit,
            // cmd::get_diff_commits,
            cmd::create_branch,
            cmd::delete_branch,
            cmd::rename_branch,
            cmd::get_branch_remote,
            cmd::get_branches_info,
            cmd::branch_compare_upstream,
            cmd::checkout_branch,
            cmd::checkout_remote_branch,
            cmd::tag,
            cmd::stash,
            cmd::blame,
            cmd::test_progress,
        ])
        .setup(|app| {
            // set window size
            let win = app.get_window("main").unwrap();
            win.set_size(
                Size::Physical(PhysicalSize{width: 900, height: 800})
            ).unwrap();

            // emit received progress message to window
            std::thread::spawn(move || {
                loop {
                    match rx_git.recv() {
                        Ok(payload) => {
                            println!("{:?}", payload);
                            win.emit("PROGRESS", payload).unwrap();
                        },
                        Err(e) => {
                            log::error!( "progress receiver error: {}", e);
                        }
                    }
                }
            });

            // set state data
            let app_data = AppData {
                settings: settings,
                repo_path: None,
                tx_git: tx_git
            };
            app.manage(ArcAppData::new(app_data));

            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
