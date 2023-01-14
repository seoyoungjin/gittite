#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::app_data::{AppData, ArcAppData};
use crate::git_api::RemoteProgress;
use tauri::Manager;
use tauri::{PhysicalSize, Size};

mod app_data;
mod args;
mod cmd;
mod git_api;
mod menu;
mod settings;

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

    let cliargs = args::process_cmdline().unwrap();
    log::trace!("cliargs: {:?}", cliargs.repo_path);
    let settings = match settings::Settings::load() {
        Ok(v) => v,
        Err(_e) => settings::Settings::default(),
    };

    // progress message
    let (tx_git, rx_git) = std::sync::mpsc::channel::<RemoteProgress>();

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(menu::create_menu("Gittite"))
        .on_menu_event(menu::event_handler)
        .invoke_handler(tauri::generate_handler![
            app_data::get_settings,
            app_data::save_settings,
            app_data::get_prop,
            app_data::set_prop,
            // repository
            cmd::is_git_repository,
            cmd::get_repo_info,
            cmd::set_repository,
            cmd::clone,
            cmd::init,
            // commit
            cmd::commit,
            cmd::commit_amend,
            cmd::commit_info,
            cmd::commit_files,
            // stage/reset
            cmd::stage_add_all,
            cmd::stage_add_path,
            cmd::stage_remove_path,
            cmd::reset_stage,
            cmd::reset_workdir,
            // log/status
            cmd::get_commits,
            cmd::get_status,
            cmd::get_remotes,
            // diff
            cmd::get_diff,
            cmd::get_diff_commit,
            // cmd::get_diff_commits,
            // branch
            cmd::branch_compare_upstream,
            cmd::checkout_branch,
            cmd::checkout_remote_branch,
            cmd::create_branch,
            cmd::delete_branch,
            cmd::get_branch_name,
            cmd::get_branch_remote,
            cmd::get_branches_info,
            cmd::rename_branch,
            cmd::validate_branch_name,
            // tag/stash/blame
            cmd::tag,
            cmd::stash,
            cmd::blame,
            // cred
            cmd::need_username_password,
            cmd::extract_username_password,
            // config/ignore
            cmd::add_to_ignore,
            // test
            cmd::test_progress,
        ])
        .setup(|app| {
            // set window size
            let win = app.get_window("main").unwrap();
            win.set_size(Size::Physical(PhysicalSize {
                width: 900,
                height: 800,
            }))
            .unwrap();

            // emit received progress message to window
            std::thread::spawn(move || loop {
                match rx_git.recv() {
                    Ok(payload) => {
                        // log::trace!("rx_git {:?}", payload);
                        win.emit("PROGRESS", payload).unwrap();
                    }
                    Err(e) => {
                        log::error!("progress receiver error: {}", e);
                        error_popup_main_thread(e.to_string());
                    }
                }
            });

            // set state data
            let app_data = AppData {
                settings: settings,
                repo_path: Some(cliargs.repo_path),
                tx_git: tx_git,
                modal: false,
            };
            app.manage(ArcAppData::new(app_data));

            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
