mod platform;
mod web;
mod mcp;
use std::sync::Arc;

use mcp::*;
use platform::*;
use tauri::{tray::TrayIconBuilder, window::Color, App, Manager, WebviewWindowBuilder};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_positioner::{Position, WindowExt};
use tokio::sync::Mutex;
use web::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name_n: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name_n)
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
fn build_first_webview(app: &mut App) {
    WebviewWindowBuilder::new(app, "home", tauri::WebviewUrl::App("/home".into()))
        .title("智能助手")
        .inner_size(300.0, 100.0)
        .decorations(false)
        .transparent(true)
        .maximizable(false)
        .build().unwrap();
}

#[cfg(target_os="macos")]
fn build_first_webview(app: &mut App) {
    use cocoa::appkit::{NSColor, NSWindow};
    use cocoa::base::{id, nil};
    use tauri::TitleBarStyle;
    let window = WebviewWindowBuilder::new(app, "home", tauri::WebviewUrl::App("/home".into()))
        .title("智能助手")
        .inner_size(300.0, 100.0)
        .decorations(false)
        .maximizable(false)
        .title_bar_style(TitleBarStyle::Transparent)
        .build().unwrap();

    let ns_window = window.ns_window().unwrap() as id;
    unsafe {
        let bg_color = NSColor::colorWithRed_green_blue_alpha_(
            nil,
            0.0,
            0.0,
            0.0,
            0.0,
        );
        ns_window.setBackgroundColor_(bg_color);
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(MCPInfo {
            client: Arc::new(Mutex::new(MCPClient::new()))
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(tauri_plugin_log::Builder::new()
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::LogDir {
                file_name: Some("logs".to_string()),
            },
        ))
        .max_file_size(50_000 /* bytes */)
        .build())
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
            get_project_root_path,
            add_servers,
            get_tools,
            call_tool,
        ])
        // This is required to get tray-relative positions to work
        .setup(|app| {
            // 创建第一个 webview，指向主页
            build_first_webview(app);
            if let Some(window) = app.get_webview_window("home") {
                window.move_window(Position::RightCenter).expect("移动窗口失败");
                window.set_background_color(Some(Color(0, 0, 0, 0))).expect("设置背景颜色失败");
            } else {
                println!("创建 home 失败");
            }
            TrayIconBuilder::new()
                .on_tray_icon_event(|app, event| {
                    tauri_plugin_positioner::on_tray_event(app.app_handle(), &event);
                })
                .build(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
