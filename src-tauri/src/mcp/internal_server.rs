use std::{collections::HashMap, sync::Arc, fmt::Debug};

use async_trait::async_trait;
use rmcp::model::{CallToolResult, JsonObject, Tool};

use super::server_operation::*;

#[async_trait]
pub trait InternalFunctionCall {
    async fn call(&self, arg: Option<JsonObject>) -> Result<CallToolResult, String>;
}

pub struct InternalFunction {
    pub name: String,
    pub desc: String,
    pub func: Box<dyn InternalFunctionCall + Send + Sync>,
    pub input_schema: Arc<JsonObject>,
}

pub struct InternalMCPServer {
    pub name: String,
    pub desc: String,
    pub tools: HashMap<String, InternalFunction>,
    pub enable: bool,
}

impl InternalMCPServer {
    pub fn set_tool(&mut self, func: InternalFunction) {
        self.tools.insert(func.name.clone(), func);
    }
}

impl Debug for InternalMCPServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InternalMCPServer").field("name", &self.name).field("desc", &self.desc).field("enable", &self.enable).finish()
    }
}

#[async_trait]
impl ServerOperation for InternalMCPServer {
    async fn connect(&mut self)->Result<(), Box<dyn std::error::Error>>{
        Ok(())
    }

    async fn disconnect(&mut self)->Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn get_tools(&mut self)->Result<Vec<Tool>, String> {
        let mut tools = Vec::new();
        for (name, tool) in self.tools.iter() {
            let name = self.name.clone() + "_" + name;
            tools.push(Tool::new(name, tool.desc.clone(), tool.input_schema.clone()));
        }
        Ok(tools)
    }

    async fn call_tool(&mut self, name: String, args: Option<JsonObject>) ->Result<CallToolResult, String> {
        if self.tools.contains_key(&name) {
            let func = self.tools.get(&name).unwrap();
            func.func.call(args).await
        }
        else {
            Err(format!("未找到工具：{}", name))
        }
    }

    fn get_config(&self)->MCPServerConfig {
        MCPServerConfig {
            name: self.name.clone(),
            command: "".into(),
            desc: self.desc.clone(),
            args: None,
            env: None,
            enable: true,
            internal: true,
        }
    }

    fn is_connected(&self)->bool {
        true
    }

    fn is_internal(&self)->bool {
        true
    }

    async fn update_config(&mut self, config: MCPServerConfig) {
        self.name = config.name;
        self.desc = config.desc;
    }

    fn get_display_info(&self)->ServerDisplayInfo {
        ServerDisplayInfo {
            name: self.name.clone(),
            command: "".into(),
            desc: self.desc.clone(),
            args: None,
            env: None,
            enable: true,
            internal: true,
            connected: true,
        }
    }
}