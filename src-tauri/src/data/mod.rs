mod server_data;

use std::{collections::HashMap, fs};
use serde_json;
use directories::ProjectDirs;
use crate::{mcp::MCPServerConfig, model::ModelData};
pub use server_data::*;

const MODEL_FILE_NAME: &str = "modeldata.json";
const SERVER_FILE_NAME: &str = "server.json";

#[tauri::command]
pub fn store_model_data(data: ModelData)->Result<(), String> {
    let project_dirs = ProjectDirs::from("", "", "assistant")
        .ok_or_else(|| "无法获取项目目录")?;

    let config_dir = project_dirs.config_dir();
    let res = fs::create_dir_all(config_dir);
    if res.is_err() {
        return Err(res.unwrap_err().to_string());
    }
    
    let file_path = config_dir.join(MODEL_FILE_NAME);
    let json = serde_json::to_string_pretty(&data)
        .map_err(|e| e.to_string())?;
    
    let res = fs::write(file_path, json);
    if res.is_err() {
        return Err(res.unwrap_err().to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn load_model_data()->Result<ModelData, ()> {
    let project_dirs = ProjectDirs::from("", "", "assistant")
        .ok_or_else(|| ())?;

    let config_dir = project_dirs.config_dir();
    let path = config_dir.join(MODEL_FILE_NAME);
    match fs::read_to_string(path) {
        Ok(content) => {
            let data: ModelData = serde_json::from_str(&content).unwrap();
            Ok(data)
        }
        Err(_) => Err(())
    }
}

#[tauri::command]
pub fn load_server_data()->Result<ServerData, String> {
    let project_dirs = ProjectDirs::from("", "", "assistant")
        .ok_or_else(|| "无法获取项目目录".to_string())?;

    let config_dir = project_dirs.config_dir();
    let path = config_dir.join(SERVER_FILE_NAME);
    match fs::read_to_string(path) {
        Ok(content) => {
            let data: ServerData = serde_json::from_str(&content)
                .map_err(|e| e.to_string())?;
            Ok(data)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err("Server config file not found".to_string())
        }
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub fn store_server_data(data: &ServerData) -> Result<(), String> {
    let project_dirs = ProjectDirs::from("", "", "assistant")
        .ok_or_else(|| "无法获取项目目录")?;

    let config_dir = project_dirs.config_dir();
    let res = fs::create_dir_all(config_dir);
    if res.is_err() {
        return Err(res.unwrap_err().to_string());
    }

    let file_path = config_dir.join(SERVER_FILE_NAME);
    let json = serde_json::to_string_pretty(&data)
        .map_err(|e| e.to_string())?;
    
    let res = fs::write(file_path, json);
    if res.is_err() {
        return Err(res.unwrap_err().to_string());
    }
    Ok(())
}
