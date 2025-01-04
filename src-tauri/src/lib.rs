mod platform;
mod web;
use platform::*;
use web::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name_n: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name_n)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            exec_cmd,
            open_app_by_name,
            get_all_app_names,
            open_app_by_shortcut,
            ask_window,
            search_duck_duck_go,
            read_text_file,
            read_text_file_at_project_root,
            write_text_file_at_project_root,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
