use std::{collections::HashMap, time::Duration};

use rmcp::model::{Tool};
use tauri::{AppHandle};
use tokio::time::Instant;

use crate::{assistant::{Assistant, RoleCard, RoleCardStoreData, APP_HANDLE, ASSISTANT, ASSISTANT_NAME}, data::{load_rolecard_data, store_rolecard_data}, mcp::{MCPServerConfig, ServerDisplayInfo, MCP_CLIENT}, model::ModelData};


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
pub async fn get_servers()->Vec<ServerDisplayInfo> {
    let client = MCP_CLIENT.lock().await;
    client.get_servers_display()
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

#[tauri::command]
pub async fn start_timer(app: AppHandle) {
    {
        let mut a = APP_HANDLE.lock().await;
        *a = Some(app.clone());
    }
    tokio::spawn(async move {
        let interval;
        {
            let ass: tokio::sync::MutexGuard<'_, Assistant> = ASSISTANT.lock().await;
            interval = Duration::from_secs(ass.think.config.pulse_interval);
        }
        let mut it = tokio::time::interval_at(Instant::now() + Duration::from_secs(5), interval);
        loop {
            it.tick().await;
            {
                let mut ass: tokio::sync::MutexGuard<'_, Assistant> = ASSISTANT.lock().await;
                ass.think_pulse().await;
            }
        }
    }).await.unwrap();
}

#[tauri::command]
pub async fn get_rolecard()->Vec<RoleCard> {
    let mut ret = Vec::new();
    if let Ok(data) = load_rolecard_data() {
        for (_, card) in data.cards {
            ret.push(card);
        }
    }
    ret
}

#[tauri::command]
pub async fn set_rolecard(card: RoleCard) {
    if card.name == ASSISTANT_NAME {
        let mut ass = ASSISTANT.lock().await;
        ass.think.set_rolecard(card.clone());
    }
    if let Ok(mut data) = load_rolecard_data() {
        data.cards.insert(card.name.clone(), card);
        store_rolecard_data(&data).unwrap();
    }
    else {
        let mut data = RoleCardStoreData {cards: HashMap::new()};
        data.cards.insert(card.name.clone(), card);
        store_rolecard_data(&data).unwrap();
    }
}