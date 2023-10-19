#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod entity;
mod start_page;
mod utils;
mod init;

use tauri::Manager;
use window_vibrancy::{apply_acrylic, apply_mica};
use crate::start_page::start;
use crate::init::windows;

fn main() {
    tauri::Builder::default()
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
        .invoke_handler(tauri::generate_handler![start::start_yunzai_and_api,windows::is_win11])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}