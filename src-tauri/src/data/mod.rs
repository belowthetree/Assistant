mod mcp;

use std::{fs};
use log::debug;
use serde_json;
use directories::ProjectDirs;
use crate::{assistant::{RoleCardStoreData}, model::ModelData};
pub use mcp::*;

const MODEL_FILE_NAME: &str = "model.json";
const SERVER_FILE_NAME: &str = "mcp.json";
const ROLECARD_FILE_NAME: &str = "rolecard.json";

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
            let t = serde_json::from_str(&content);
            if t.is_err() {
                debug!("{:?}", t)
            }
            let data: ModelData = t.unwrap();
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

pub fn store_rolecard_data(data: &RoleCardStoreData)->Result<(), String> {
    let project_dirs = ProjectDirs::from("", "", "assistant")
        .ok_or_else(|| "无法获取项目目录")?;

    let config_dir = project_dirs.config_dir();
    let res = fs::create_dir_all(config_dir);
    if res.is_err() {
        return Err(res.unwrap_err().to_string());
    }

    let file_path = config_dir.join(ROLECARD_FILE_NAME);
    let json = serde_json::to_string_pretty(&data)
        .map_err(|e| e.to_string())?;
    
    let res = fs::write(file_path, json);
    if res.is_err() {
        return Err(res.unwrap_err().to_string());
    }
    Ok(())
}

pub fn load_rolecard_data()->Result<RoleCardStoreData, String> {
    let project_dirs = ProjectDirs::from("", "", "assistant")
        .ok_or_else(|| "无法获取项目目录".to_string())?;

    let config_dir = project_dirs.config_dir();
    let path = config_dir.join(ROLECARD_FILE_NAME);
    match fs::read_to_string(path) {
        Ok(content) => {
            let data: RoleCardStoreData = serde_json::from_str(&content)
                .map_err(|e| e.to_string())?;
            Ok(data)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err("Server config file not found".to_string())
        }
        Err(e) => Err(e.to_string())
    }
}