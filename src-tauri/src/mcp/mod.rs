mod client;
mod server;
use std::sync::Arc;

pub use client::*;
use rmcp::model::{CallToolResult, JsonObject, Tool};
pub use server::*;
use tauri::State;
use tokio::sync::Mutex;

pub struct MCPInfo {
    pub client: Arc<Mutex<MCPClient>>
}

#[tauri::command]
pub async fn add_servers(configs: Vec<MCPServerConfig>, state: State<'_, MCPInfo>)->Result<(), ()> {
    let mut clt = state.client.lock().await;
    for cfg in configs {
        clt.add_server(cfg);
    }
    Ok(())
}

#[tauri::command]
pub async fn get_tools(name: String, state: State<'_, MCPInfo>)->Result<Vec<Tool>, String> {
    let mut clt = state.client.lock().await;
    clt.get_tools(name).await
}

#[tauri::command]
pub async fn call_tool(server_name: String, tool_name: String, args: Option<JsonObject>, state: State<'_, MCPInfo>)->Result<CallToolResult, String> {
    let mut clt = state.client.lock().await;
    clt.call_tool(server_name, tool_name, args).await
}