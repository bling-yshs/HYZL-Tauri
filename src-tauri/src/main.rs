#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod entity;
mod utils;
mod page;

use tauri::Manager;
use window_vibrancy::{ apply_mica};

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
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            close_splashscreen,
            page::start::start::start_yunzai_and_api,
            page::issue_fix::issue_fix::reinstall_dependence,
            page::start::start::start_yunzai,
            page::start::start::download_sign_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}