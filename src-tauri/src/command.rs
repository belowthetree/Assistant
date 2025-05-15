use rmcp::model::{Tool};
use tauri::{AppHandle, Emitter};

use crate::{assistant::{Assistant, APP_HANDLE, ASSISTANT}, event::SYSTEM_NOTIFY, mcp::{MCPServer, MCPServerConfig, MCP_CLIENT}, model::ModelData};


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
pub async fn set_server(server: MCPServerConfig)->Result<(), ()> {
    let mut client = MCP_CLIENT.lock().await;
    client.set_server(&server).await;
    Ok(())
}

#[tauri::command]
pub async fn get_servers()->Vec<MCPServer> {
    let client = MCP_CLIENT.lock().await;
    client.get_servers()
}

#[tauri::command]
pub async fn get_tools(name: String)->Result<Vec<Tool>, String> {
    let mut client = MCP_CLIENT.lock().await;
    client.get_tools(name).await
}

#[tauri::command]
pub async fn talk(ctx: String, app: AppHandle)->Result<String, String> {
    {
        let mut a = APP_HANDLE.lock().await;
        *a = Some(app.clone());
    }
    let mut ass = ASSISTANT.lock().await;
    let res = ass.talk(ctx, app).await;
    if res.is_ok() {
        let ret = res.unwrap();
        if let Some(tools) = ret.tool_calls {
            for tool in tools.iter() {
                let mut client = MCP_CLIENT.lock().await;
                client.call_tool(tool.clone()).await.unwrap();
            }
        }
        Ok(ret.content)
    }
    else {
        Err(res.unwrap_err())
    }
}

#[tauri::command]
pub async fn update_model() {
    let mut ass: tokio::sync::MutexGuard<'_, Assistant> = ASSISTANT.lock().await;
    ass.refresh_model_data();
}