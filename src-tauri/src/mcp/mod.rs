mod client;
mod server;
use std::sync::Arc;

pub use client::*;
use log::debug;
use rmcp::model::{CallToolResult, JsonObject, Tool};
pub use server::*;
use tauri::State;
use tokio::sync::Mutex;
use lazy_static::lazy_static;

pub struct MCPInfo {
    pub client: Arc<Mutex<MCPClient>>
}

lazy_static! {
    pub static ref MCP_CLIENT: Arc<Mutex<MCPClient>> = Arc::new(Mutex::new(MCPClient::new()));
}

pub fn init() {
    debug!("初始化 mcp");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = MCP_CLIENT.lock().await;
        debug!("加载 mcp 配置");
        client.refresh_mcp_config().await;
        debug!("初始化结束");
    });
}