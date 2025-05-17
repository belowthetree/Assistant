use std::{collections::HashMap, fmt::Debug};

use async_trait::async_trait;
use rmcp::model::{CallToolResult, JsonObject, Tool};
use serde::{Deserialize, Serialize};

pub fn _default_true()->bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MCPServerConfig {
    pub name: String,
    pub command: String,
    pub desc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(default = "_default_true")]
    pub enable: bool,
    // 是否是内部提供的服务
    #[serde(default)]
    pub internal: bool
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerDisplayInfo {
    pub name: String,
    pub command: String,
    pub desc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(default = "_default_true")]
    pub enable: bool,
    // 是否是内部提供的服务
    #[serde(default)]
    pub internal: bool,
    pub connected: bool,
}

#[async_trait]
#[allow(unused)]
pub trait ServerOperation: Debug {
    async fn connect(&mut self)->Result<(), Box<dyn std::error::Error>>;
    async fn disconnect(&mut self)->Result<(), Box<dyn std::error::Error>>;
    async fn get_tools(&mut self)->Result<Vec<Tool>, String>;
    async fn call_tool(&mut self, name: String, args: Option<JsonObject>) ->Result<CallToolResult, String>;
    fn get_config(&self)->MCPServerConfig;
    fn is_connected(&self)->bool;
    fn is_internal(&self)->bool;
    async fn update_config(&mut self, config: MCPServerConfig);
    fn get_display_info(&self)->ServerDisplayInfo;
}