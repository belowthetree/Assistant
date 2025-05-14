use rmcp::model::{CallToolResult, JsonObject, Tool};
use tauri::State;

use crate::{assistant::ASSISTANT, mcp::{MCPServer, MCPServerConfig, MCP_CLIENT}, model::ModelData};


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
pub async fn call_tool(server_name: String, tool_name: String, args: Option<JsonObject>)->Result<CallToolResult, String> {
    let mut client = MCP_CLIENT.lock().await;
    client.call_tool(server_name, tool_name, args).await
}