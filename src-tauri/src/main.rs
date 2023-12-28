// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// define print
#[tauri::command]
fn print(message: String) {
    println!("{}", message);
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![print])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
