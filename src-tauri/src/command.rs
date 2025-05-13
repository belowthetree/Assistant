use tauri::State;

use crate::{assistant::ASSISTANT, mcp::{MCPInfo, MCPServerConfig}, model::ModelData};


#[tauri::command]
pub async fn get_models()->Result<Vec<String>, String> {
    let ass = ASSISTANT.lock().await;
    ass.get_models().await
}

#[tauri::command]
pub async fn set_model(data: ModelData) {
    let mut ass = ASSISTANT.lock().await;
    ass.set_model_data(data);
    ass.store_server_data();
}

#[tauri::command]
pub async fn add_server(server: MCPServerConfig, state: State<'_, MCPInfo>)->Result<(), ()> {
    let mut clt = state.client.lock().await;
    clt.add_server(&server);
    Ok(())
}