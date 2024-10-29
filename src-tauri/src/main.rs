#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod text_processor;

use std::io::Cursor;

#[tauri::command]
async fn capture_screenshot(x: i32, y: i32, width: u32, height: u32) -> Result<Vec<u8>, String> {
    use screenshots::Screen;
    
    let screens = Screen::all().map_err(|e| e.to_string())?;
    let screen = screens.first().ok_or("No screen found")?;
    
    let image = screen.capture_area(x, y, width, height)
        .map_err(|e| e.to_string())?;
    
    // 创建一个内存缓冲区来存储 PNG 数据
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    image.write_to(&mut cursor, screenshots::image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;
    
    Ok(buffer)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            text_processor::process_text,
            text_processor::get_embedded_fonts,
            capture_screenshot
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
