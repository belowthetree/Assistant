//mod character;
mod client;
mod internal_server;
mod schedule;
mod server;
mod server_operation;
mod systemutil;

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
pub use client::*;
use internal_server::{InternalFunction, InternalFunctionCall, InternalMCPServer};
use lazy_static::lazy_static;
use log::debug;
pub use schedule::*;
pub use server::*;
pub use server_operation::*;
use systemutil::SystemUtilServer;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref MCP_CLIENT: Arc<Mutex<MCPClient>> = Arc::new(Mutex::new(MCPClient::new()));
}

#[async_trait]
pub trait InternalServerInfo {
    fn get_server(&self) -> InternalMCPServer;
}

pub fn init() {
    debug!("初始化 mcp");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = MCP_CLIENT.lock().await;
        debug!("加载 mcp 配置");
        client.refresh_mcp_config().await;
        client
            .set_internal_servers(get_internal_servers(vec![
                Box::new(SystemUtilServer {}),
                Box::new(ScheUtilServer {}),
            ]))
            .await;
        let tools = client.get_all_tools().await.unwrap();
        for tool in tools {
            debug!("tool: {}: {}", tool.name, tool.description);
        }
        debug!("初始化 mcp 结束");
    });
}

// 所有的内部 server 从这里添加
pub fn get_internal_servers(
    infos: Vec<Box<dyn InternalServerInfo>>,
) -> HashMap<String, InternalMCPServer> {
    let mut servers = HashMap::new();
    for info in infos {
        let server = info.get_server();
        servers.insert(server.name.clone(), server);
    }
    servers
}

pub fn get_server(
    name: &str,
    desc: &str,
    tools: Vec<Box<dyn InternalFunctionCall + Send + Sync + 'static>>,
) -> InternalMCPServer {
    let mut server = InternalMCPServer {
        name: name.into(),
        desc: desc.into(),
        tools: HashMap::new(),
        enable: true,
    };

    for tool in tools {
        server.set_tool(InternalFunction {
            name: tool.get_name(),
            desc: tool.get_desc(),
            input_schema: tool.get_input_schema(),
            func: tool,
        });
    }
    server
}
