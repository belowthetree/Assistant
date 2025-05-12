use crate::{assistant::ASSISTANT, model::ModelData};


#[tauri::command]
pub async fn get_models()->Result<Vec<String>, String> {
    let ass = ASSISTANT.lock().await;
    ass.get_models().await
}

#[tauri::command]
pub async fn set_model(data: ModelData) {
    let mut ass = ASSISTANT.lock().await;
    ass.set_model_data(data);
}