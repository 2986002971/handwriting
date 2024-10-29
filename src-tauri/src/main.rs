#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod text_processor;
use crate::text_processor::process_text;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![process_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
