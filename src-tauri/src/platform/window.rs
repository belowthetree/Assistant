use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct AskWindowInfo<'a> {
  msg: &'a str,
  title: &'a str,
}

#[tauri::command]
pub fn ask_window(app: AppHandle, msg: &str, title: &str) {
    app.emit("ask_window", AskWindowInfo {
        msg,
        title
    }).unwrap();
}