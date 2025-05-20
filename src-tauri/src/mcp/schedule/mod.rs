use std::sync::Arc;

use async_trait::async_trait;
use chrono::Local;
use log::{debug, warn};
use rmcp::model::{CallToolResult, JsonObject};
use sche_item::{EScheStatus, ScheItem};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::data::load_data;
use lazy_static::lazy_static;

use super::{get_server, internal_server::InternalFunctionCall, InternalServerInfo};

mod sche_item;

lazy_static! {
    pub static ref SCHEDULE: Arc<Mutex<Schedule>> = Arc::new(Mutex::new(Schedule::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub schedules: Vec<ScheItem>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            schedules: Vec::new(),
        }
    }

    pub fn load_data(&mut self) {
        let data = load_data::<Schedule>();
        match data {
            Ok(data) => {
                self.schedules = data.schedules;
            }
            Err(err) => {
                warn!("{:?}", err);
            }
        }
    }

    pub fn add_sche(&mut self, sche: ScheItem) {
        if self.schedules.len() <= 0 {
            self.load_data();
        }
        self.schedules.push(sche);
    }
}

pub struct ScheUtilServer {}

impl InternalServerInfo for ScheUtilServer {
    fn get_server(&self) -> super::internal_server::InternalMCPServer {
        get_server(
            "ScheduleUtil",
            "操作用户日程",
            vec![Box::new(ScheUtilAddSche {})],
        )
    }
}
pub struct ScheUtilAddSche {}

#[async_trait]
impl InternalFunctionCall for ScheUtilAddSche {
    async fn call(&self, args: Option<JsonObject>) -> Result<CallToolResult, String> {
        debug!("AddSche {:?}", args);
        if let Some(js) = args {
            if !js.contains_key("title") || !js.contains_key("content") {
                return Err("AddSche no key".into());
            }
            let title = js.get("title").unwrap().as_str().unwrap().to_string();
            let content = js.get("content").unwrap().as_str().unwrap().to_string();
            let data = ScheItem {
                title,
                content,
                status: EScheStatus::Unfinish,
                create_time: Local::now(),
                finish_time: Local::now(),
            };
            let mut sche = SCHEDULE.lock().await;
            sche.add_sche(data);
            return Ok(CallToolResult {
                content: Vec::new(),
                is_error: None,
            });
        }
        Err("".into())
    }

    fn get_input_schema(&self) -> Arc<JsonObject> {
        Arc::new(
            serde_json::from_str(
                r#"
        {
            "properties":{
                "title":{
                    "description":"日程标题",
                    "type": "string"
                },
                "content":{
                    "description":"日程内容",
                    "type": "string"
                }
            }
        }"#,
            )
            .unwrap(),
        )
    }

    fn get_desc(&self) -> String {
        "添加日程安排".into()
    }

    fn get_name(&self) -> String {
        "add_sche".into()
    }
}
