mod client;
mod server;
mod internal_server;
mod server_operation;

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
pub use client::*;
use internal_server::{InternalFunction, InternalFunctionCall, InternalMCPServer};
use log::{debug, warn};
use rmcp::model::{CallToolResult, JsonObject};
pub use server::*;
use tauri::Emitter;
use tokio::sync::Mutex;
use lazy_static::lazy_static;
pub use server_operation::*;

use crate::{assistant::APP_HANDLE, event::SYSTEM_NOTIFY};

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
        client.set_internal_servers(get_internal_servers()).await;
        let tools = client.get_all_tools().await.unwrap();
        for tool in tools {
            debug!("tool: {}: {}", tool.name, tool.description);
        }
        debug!("初始化 mcp 结束");
    });
}

// 所有的内部 server 从这里添加
pub fn get_internal_servers()->HashMap<String, InternalMCPServer> {
    let mut servers = HashMap::new();
    let mut server = InternalMCPServer {
        name: "systemutil".into(),
        desc: "use system api to help user".into(),
        tools: HashMap::new(),
        enable: true,
    };
    server.set_tool(InternalFunction {
        name: "notify".into(),
        desc: "system notify".into(),
        func: Box::new(SystemUtilNotify{}),
        input_schema: Arc::new(serde_json::from_str(r#"
        {
            "properties":{
                "content":{
                    "description":"send a system notify to user",
                    "type": "string"
                }
            }
        }"#).unwrap())
    });
    servers.insert("systemutil".into(), server);
    servers
}

pub struct SystemUtilNotify {}

#[async_trait]
impl InternalFunctionCall for SystemUtilNotify {
    async fn call(&self, args: Option<JsonObject>) -> Result<CallToolResult, String> {
        debug!("input {:?}", args);
        if let Some(args) = args {
            if args.contains_key("content") {
                let content = args.get("content").unwrap();
                if !content.is_string() {
                    return Err("content 不是字符串".into());
                }
                let content = content.as_str().unwrap();
                debug!("system notify {}", content);
                let app = APP_HANDLE.lock().await;
                if let Some(app) = &*app {
                    debug!("send {}", SYSTEM_NOTIFY);
                    app.emit(SYSTEM_NOTIFY, content).unwrap();
                }
                else {
                    warn!("no app");
                }
                return Ok(CallToolResult { content: Vec::new(), is_error: None });
            }
        }
        Err("notify args error".into())
    }
}