#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use sha2::{Digest, Sha256};
use tauri::Manager;
use window_vibrancy::apply_mica;
use app::DataResponse;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            let window = app.get_window("main").unwrap();
            window.unminimize().unwrap();
            window.set_focus().unwrap();
        }))
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
            calc_sha256,
            copy_directory,
            // page::start::start::start_yunzai_and_api,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

#[tauri::command]
async fn calc_sha256(path: String) -> DataResponse<String> {
    let path = PathBuf::from(path);
    let input = match File::open(path) {
        Ok(res) => { res }
        Err(e) => { return DataResponse::failure(e.to_string()); }
    };
    let mut reader = BufReader::new(input);
    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer).unwrap();
            if count == 0 { break; }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    return DataResponse::success(format!("{:x}", digest));
}

#[tauri::command]
async fn copy_directory(source: String, destination: String) -> DataResponse<String> {
    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true;
    return match fs_extra::dir::copy(source, destination, &options) {
        Ok(_) => {
            DataResponse::fast_success()
        }
        Err(v) => {
            DataResponse::failure(v.to_string())
        }
    };
}