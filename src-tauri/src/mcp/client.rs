use std::collections::HashMap;

use log::debug;
use rmcp::model::{CallToolResult, JsonObject, Tool};

use super::{MCPServer, MCPServerConfig};

pub struct MCPClient {
    pub servers: HashMap<String, MCPServer>,
    pub max_server_num: usize,
    pub connted_server_num: usize,
}

impl MCPClient {
    pub fn new() ->Self {
        Self {
            servers: HashMap::new(),
            max_server_num: 10,
            connted_server_num: 0,
        }
    }

    pub fn add_server(&mut self, config: MCPServerConfig) {
        debug!("添加服务 {:?}", config);
        if !self.servers.contains_key(&config.name) {
            let mut server = MCPServer::new(config);
            if self.connted_server_num < self.max_server_num {
                let _ = server.connect();
            }
            self.servers.insert(server.config.name.clone(), server);
        }
    }

    fn get_server(&mut self, name: String)->Result<&mut MCPServer, ()> {
        if self.servers.contains_key(&name) {
            return Ok(self.servers.get_mut(&name).unwrap());
        }
        Err(())
    }

    pub async fn call_tool(&mut self, server_name: String, tool_name: String, args: Option<JsonObject>) -> Result<CallToolResult, String> {
        let res = self.get_server(server_name.clone());
        if res.is_err() {
            return Err(format!("服务 {} 未找到", server_name));
        }
        let server = res.unwrap();
        if !server.connected {
            return Err(format!("服务 {} 未连接", server_name));
        }
        let ret = server.call_tool(tool_name, args).await;
        let mut count = 0;
        for (_, server) in self.servers.iter().by_ref() {
            if server.call_count > count {
                count = server.call_count;
            }
        }
        for (_, server) in self.servers.iter_mut().by_ref() {
            if server.call_count < count {
                let _ = server.disconnect();
            }
        }
        ret
    }

    pub async fn get_tools(&mut self, name: String) -> Result<Vec<Tool>, String> {
        debug!("获取 tools {}", name);
        let res = self.get_server(name.clone());
        if res.is_err() {
            return Err(format!("服务 {} 未找到", name));
        }
        let server = res.unwrap();
        server.get_tools().await
    }
}