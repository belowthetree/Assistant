use async_trait::async_trait;
use log::debug;
use rmcp::{
    model::{CallToolRequestParam, CallToolResult, JsonObject, Tool},
    service::{DynService, RunningService},
    transport::TokioChildProcess,
    RoleClient, ServiceExt,
};
use std::{borrow::Cow, fmt::Debug};
use tokio::process::Command;

use super::{
    server_operation::{MCPServerConfig, ServerOperation},
    ServerDisplayInfo,
};

pub struct MCPServer {
    pub config: MCPServerConfig,
    pub connected: bool,
    pub tools: Vec<Tool>,
    pub service: Option<RunningService<RoleClient, Box<dyn DynService<RoleClient>>>>,
    pub call_count: usize,
}

impl Debug for MCPServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MCPServer")
            .field("config", &self.config)
            .field("connected", &self.connected)
            .field("tools", &self.tools)
            .field("call_count", &self.call_count)
            .finish()
    }
}

impl Clone for MCPServer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            connected: self.connected.clone(),
            tools: self.tools.clone(),
            service: None,
            call_count: self.call_count.clone(),
        }
    }
}

impl MCPServer {
    pub fn new(config: MCPServerConfig) -> Self {
        Self {
            config,
            connected: false,
            tools: Vec::new(),
            service: None,
            call_count: 0,
        }
    }
}
#[async_trait]
impl ServerOperation for MCPServer {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.connected || !self.config.enable || self.config.internal {
            return Ok(());
        }
        let mut transport = Command::new(self.config.command.clone());
        if let Some(args) = self.config.args.clone() {
            for arg in args {
                transport.arg(arg);
            }
        }
        let service = ().into_dyn().serve(TokioChildProcess::new(&mut transport)?).await?;
        let server_info = service.peer_info();
        debug!("服务器连接 {:?}", server_info);
        self.service = Some(service);
        self.connected = true;
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.connected {
            return Ok(());
        }
        if let Some(service) = self.service.take() {
            service.cancel().await?;
        }
        Ok(())
    }

    // 工具名字都拼接上了服务名：servername_toolname
    async fn get_tools(&mut self) -> Result<Vec<Tool>, String> {
        if self.tools.len() > 0 {
            return Ok(self.tools.clone());
        }
        let _ = self.connect().await;
        if self.connected {
            if let Some(service) = self.service.as_ref() {
                let res = service.list_all_tools().await;
                if let Ok(mut tools) = res {
                    for tool in tools.iter_mut() {
                        tool.name = Cow::Owned(self.config.name.to_string() + "_" + &tool.name)
                    }
                    return Ok(tools);
                }
                return Err(res.unwrap_err().to_string());
            }
        }
        Err("未连接到此服务".into())
    }

    async fn call_tool(
        &mut self,
        name: String,
        args: Option<JsonObject>,
    ) -> Result<CallToolResult, String> {
        let _ = self.connect().await;
        if self.connected {
            if let Some(service) = self.service.as_ref() {
                self.call_count += 1;
                let param = CallToolRequestParam {
                    name: name.into(),
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

    fn get_config(&self) -> MCPServerConfig {
        self.config.clone()
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn is_internal(&self) -> bool {
        false
    }

    async fn update_config(&mut self, config: MCPServerConfig) {
        self.config = config;
        self.disconnect().await.unwrap();
        self.connect().await.unwrap();
    }

    fn get_display_info(&self) -> ServerDisplayInfo {
        ServerDisplayInfo {
            name: self.config.name.clone(),
            command: self.config.command.clone(),
            desc: self.config.desc.clone(),
            args: self.config.args.clone(),
            env: self.config.env.clone(),
            enable: self.config.enable,
            internal: false,
            connected: self.connected,
        }
    }
}
