use std::collections::HashMap;
use log::debug;
use rmcp::{model::{CallToolRequestParam, CallToolResult, JsonObject, Tool}, service::{DynService, RunningService}, transport::TokioChildProcess, RoleClient, ServiceExt};
use serde::{Serialize, Deserialize};
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MCPServerConfig {
    pub name: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

pub struct MCPServer {
    pub config: MCPServerConfig,
    pub connected: bool,
    pub tools: Vec<Tool>,
    pub service: Option<RunningService<RoleClient, Box<dyn DynService<RoleClient>>>>,
    pub call_count: usize,
}

impl MCPServer {
    pub fn new(config: MCPServerConfig)->Self {
        Self {
            config,
            connected: false,
            tools: Vec::new(),
            service: None,
            call_count: 0,
        }
    }
    pub async fn connect(&mut self)->Result<(), Box<dyn std::error::Error>> {
        if self.connected {
            return Ok(())
        }
        let mut transport = Command::new(self.config.command.clone());
        if let Some(args) = self.config.args.clone() {
            for arg in args {
                transport.arg(arg);
            }
        }
        let service = ().into_dyn().serve(
            TokioChildProcess::new(&mut transport)?
        ).await?;
        let server_info = service.peer_info();
        debug!("服务器连接 {:?}", server_info);
        self.service = Some(service);
        self.connected = true;
        Ok(())
    }

    pub async fn disconnect(&mut self)->Result<(), Box<dyn std::error::Error>> {
        if !self.connected {
            return Ok(())
        }
        if let Some(service) = self.service.take() {
            service.cancel().await?;
        }
        Ok(())
    }

    pub async fn get_tools(&mut self)->Result<Vec<Tool>, String> {
        if self.tools.len() > 0 {
            return Ok(self.tools.clone());
        }
        let _ = self.connect().await;
        if self.connected {
            if let Some(service) = self.service.as_ref() {
                let res = service.list_all_tools().await;
                if let Ok(tools) = res {
                    debug!("工具：{:?}", tools);
                    return Ok(tools);
                }
                return Err(res.unwrap_err().to_string());
            }
        }
        Err("未连接到此服务".into())
    }

    pub async fn call_tool(&mut self, name: String, args: Option<JsonObject>) ->Result<CallToolResult, String> {
        let _ = self.connect().await;
        if self.connected {
            if let Some(service) = self.service.as_ref() {
                self.call_count += 1;
                let param = CallToolRequestParam {
                    name : name.into(),
                    arguments: args,
                };
                let res = service.call_tool(param).await;
                if let Ok(ret) = res {
                    return Ok(ret);
                }
                return Err(res.unwrap_err().to_string());
            }
        }
        return Err("未连接到此服务".into());
    }
}