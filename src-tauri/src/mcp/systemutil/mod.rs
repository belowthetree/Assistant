use std::sync::Arc;

use async_trait::async_trait;
use chrono::Local;
use log::{debug, warn};
use rmcp::model::{CallToolResult, Content, JsonObject, RawContent};
use sysinfo::System;

use super::{
    get_server,
    internal_server::{InternalFunctionCall, InternalMCPServer},
    InternalServerInfo,
};

pub struct SystemUtilServer;

impl InternalServerInfo for SystemUtilServer {
    fn get_server(&self) -> InternalMCPServer {
        get_server(
            "SystemInfo",
            "使用一系列系统功能",
            vec![
                Box::new(SystemUtilSystemInfo {}),
                Box::new(SystemUtilNotify {}),
            ],
        )
    }
}

pub struct SystemUtilSystemInfo {}

#[async_trait]
impl InternalFunctionCall for SystemUtilSystemInfo {
    async fn call(&self, _: Option<JsonObject>) -> Result<CallToolResult, String> {
        let mut sys = System::new_all();
        // First we update all information of our `System` struct.
        sys.refresh_all();
        let ret = format!(
            r#"## 基础信息
时间：{}
## 操作系统信息
system:
// RAM and swap information:
total memory: {} bytes
used memory : {} bytes
total swap  : {} bytes
used swap   : {} bytes

// Display system information:
System name:             {:?}
System kernel version:   {:?}
System OS version:       {:?}
System host name:        {:?}

// Number of CPUs:
NB CPUs: {}
"#,
            Local::now(),
            sys.total_memory(),
            sys.used_memory(),
            sys.total_memory(),
            sys.used_memory(),
            System::name(),
            System::kernel_version(),
            System::os_version(),
            System::host_name(),
            sys.cpus().len()
        );
        Ok(CallToolResult::success(vec![Content::new(
            RawContent::text(ret),
            None,
        )]))
    }

    fn get_input_schema(&self) -> Arc<JsonObject> {
        Arc::new(
            serde_json::from_str(
                r#"
        {
            "properties":{
            }
        }"#,
            )
            .unwrap(),
        )
    }

    fn get_desc(&self) -> String {
        "获取用户操作基础系统信息".into()
    }

    fn get_name(&self) -> String {
        "get_system_info".into()
    }
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
                let res = notifica::notify("助理", content);
                if res.is_err() {
                    warn!("{:?}", res);
                }
                return Ok(CallToolResult {
                    content: Vec::new(),
                    is_error: None,
                });
            }
        }
        Err("notify args error".into())
    }

    fn get_input_schema(&self) -> Arc<JsonObject> {
        Arc::new(
            serde_json::from_str(
                r#"
        {
            "properties":{
                "content":{
                    "description":"send a system notify to user",
                    "type": "string"
                }
            }
        }"#,
            )
            .unwrap(),
        )
    }

    fn get_desc(&self) -> String {
        "向用户发送系统通知".into()
    }

    fn get_name(&self) -> String {
        "notify".into()
    }
}
