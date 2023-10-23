#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod entity;
mod utils;
mod init;
mod page;

use tauri::Manager;
use window_vibrancy::{ apply_mica};
use crate::init::windows;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(target_os = "windows")]
            match apply_mica(&window, None) {
                Ok(_) => (),
                Err(_) => ()
            }
            window.set_decorations(true).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            page::start::start::start_yunzai_and_api,
            windows::is_win11,
            page::issue_fix::issue_fix::reinstall_dependence,
            page::start::start::start_yunzai,
            page::start::start::download_sign_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}