use std::fs;
use serde_json;
use directories::ProjectDirs;
use crate::model::ModelData;

const MODEL_FILE_NAME: &str = "modeldata.json";

#[tauri::command]
pub async fn store_model_data(data: ModelData)->Result<(), String> {
    let project_dirs = ProjectDirs::from("", "", "assistant")
        .ok_or_else(|| "Could not determine project directories")?;

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
pub async fn load_model_data()->Result<ModelData, ()> {
    let project_dirs = ProjectDirs::from("", "", "assistant")
        .ok_or_else(|| ())?;

    let config_dir = project_dirs.config_dir();
    let path = config_dir.join(MODEL_FILE_NAME);
    if fs::exists(path.clone()).is_ok() {
        let res = fs::read_to_string(path);
        if res.is_ok() {
            let data: ModelData = serde_json::from_str(&res.unwrap()).unwrap();
            return Ok(data);
        }
    }
    Err(())
}