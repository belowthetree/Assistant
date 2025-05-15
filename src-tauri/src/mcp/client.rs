use std::collections::HashMap;

use log::{debug, warn};
use rmcp::{model::{CallToolResult, Tool}};

use crate::{data::{load_server_data, store_server_data, ServerData}, model::ToolCall};

use super::{internal_server::{InternalMCPServer}, MCPServer, MCPServerConfig, ServerOperation};

pub struct MCPClient {
    servers: HashMap<String, MCPServer>,
    internal_servers: HashMap<String, InternalMCPServer>
    // pub max_server_num: usize,
    // pub connted_server_num: usize,
}

impl MCPClient {
    pub fn new() ->Self {
        Self {
            servers: HashMap::new(),
            internal_servers: HashMap::new(),
            // max_server_num: 10,
            // connted_server_num: 0,
        }
    }

    pub async fn refresh_mcp_config(&mut self) {
        let cfg = load_server_data();
        if let Ok(res) = cfg {
            self.set_servers(&res.servers).await;
            debug!("读取 server 配置：{:?} {:?}", res, self.servers);
        }
    }

    pub fn set_internal_servers(&mut self, internal_server: HashMap<String, InternalMCPServer>) {
        self.internal_servers = internal_server
    }

    pub fn store_mcp_config(&self) {
        let mut cfg = ServerData::new();
        for (name, server) in self.servers.iter() {
            cfg.servers.insert(name.clone(), server.config.clone());
        }
        if let Err(res) = store_server_data(&cfg) {
            warn!("保存 mcp 出错: {}", res);
        }
    }

    pub async fn set_server(&mut self, config: &MCPServerConfig) {
        debug!("添加服务 {:?}", config);
        if !self.servers.contains_key(&config.name) {
            let mut server = MCPServer::new(config.clone());
        // if self.connted_server_num < self.max_server_num {
            //     let _ = server.connect();
            // }
            let res = server.connect().await;
            if res.is_err() {
                warn!("{:?}", res);
            }
            self.servers.insert(server.config.name.clone(), server);
        }
        else {
            let res = self.servers.get_mut(&config.name).unwrap().disconnect().await;
            if res.is_err() {
                warn!("{:?}", res);
            }
            let server = MCPServer::new(config.clone());
            self.servers.insert(server.config.name.clone(), server);
        }
        self.store_mcp_config();
    }

    pub async fn set_servers(&mut self, servers: &HashMap<String, MCPServerConfig>) {
        self.servers.clear();
        for (_, server) in servers {
            self.set_server(server).await;
        }
    }

    pub fn get_servers(&self)->Vec<MCPServer> {
        let mut servers: Vec<MCPServer> = Vec::new();
        for (_, ser) in self.servers.iter() {
            servers.push(ser.clone());
        }
        debug!("{:?} {:?}", self.servers, servers);
        return servers;
    }

    fn get_server(&mut self, name: String)->Result<&mut MCPServer, ()> {
        if self.servers.contains_key(&name) {
            return Ok(self.servers.get_mut(&name).unwrap());
        }
        Err(())
    }

    fn get_internal_server(&mut self, name: String)->Result<&mut InternalMCPServer, ()> {
        if self.internal_servers.contains_key(&name) {
            return Ok(self.internal_servers.get_mut(&name).unwrap());
        }
        Err(())
    }

    pub async fn call_tool(&mut self, tool: ToolCall) -> Result<CallToolResult, String> {
        if let Some((server_name, tool_name)) = tool.function.name.split_once("_") {
            let res = self.get_server(server_name.to_string());
            if res.is_ok() {
                let server = res.unwrap();
                let ret = server.call_tool(tool_name.to_string(), serde_json::from_str(&tool.function.arguments).unwrap()).await;
                if !server.connected {
                    return Err(format!("服务 {} 未连接", server_name));
                }
                return ret;
            }
            else {
                let internal = self.get_internal_server(server_name.to_string());
                if internal.is_ok() {
                    return internal.unwrap().call_tool(tool_name.to_string(), serde_json::from_str(&tool.function.arguments).unwrap()).await;
                }
            }
            return Err(format!("无法找到服务: {:?} {}", tool, server_name));
        }
        return Err(format!("无法分离服务名字: {:?}", tool));
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

    pub async fn get_all_tools(&mut self)->Result<Vec<Tool>, String> {
        let mut res = Vec::new();
        for (_, server) in self.servers.iter_mut() {
            if let Ok(mut tools) = server.get_tools().await {
                res.append(&mut tools);
            }
        }
        for (_, server) in self.internal_servers.iter_mut() {
            res.append(&mut server.get_tools());
        }
        Ok(res)
    }
}