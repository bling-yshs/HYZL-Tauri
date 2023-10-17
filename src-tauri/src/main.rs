// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_message,plus_five])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn get_message() -> String {
    return "Here is message from rust backend".to_string();
}

#[tauri::command]
fn plus_five(number: i32) -> i32 {
    return number + 5;
}