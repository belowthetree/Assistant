use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use rmcp::model::{CallToolResult, JsonObject, Tool};

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
}

impl InternalMCPServer {
    pub async fn call_tool(&self, name: String, args: Option<JsonObject>)->Result<CallToolResult, String> {
        if self.tools.contains_key(&name) {
            let func = self.tools.get(&name).unwrap();
            func.func.call(args).await
        }
        else {
            Err(format!("未找到工具：{}", name))
        }
    }

    pub fn set_tool(&mut self, func: InternalFunction) {
        self.tools.insert(func.name.clone(), func);
    }

    pub fn get_tools(&self)->Vec<Tool> {
        let mut tools = Vec::new();
        for (name, tool) in self.tools.iter() {
            let name = self.name.clone() + "_" + name;
            tools.push(Tool::new(name, tool.desc.clone(), tool.input_schema.clone()));
        }
        tools
    }
}