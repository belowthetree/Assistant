use std::sync::Arc;

use super::{get_server, internal_server::InternalFunctionCall, InternalServerInfo};
use crate::data::{load_data, store_data};
use async_trait::async_trait;
use chrono::{Local, NaiveDateTime};
use lazy_static::lazy_static;
use log::{debug, error, warn};
use rmcp::model::{Annotated, CallToolResult, JsonObject, RawContent, RawTextContent};
pub use sche_item::{EScheStatus, ScheItem};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

mod sche_item;

const SCHE_FILE_NAME: &str = "sche.json";

lazy_static! {
    pub static ref SCHEDULE: Arc<Mutex<Schedule>> = Arc::new(Mutex::new(Schedule::new()));
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    schedules: Vec<ScheItem>,
}

impl Schedule {
    pub fn new() -> Self {
        let mut ret = Self {
            schedules: Vec::new(),
        };
        ret.load_data();
        ret
    }

    pub fn load_data(&mut self) {
        let data = load_data::<Schedule>(&SCHE_FILE_NAME.to_string());
        debug!("加载日程：{:?}", data);
        match data {
            Ok(data) => {
                self.schedules = data.schedules;
            }
            Err(err) => {
                warn!("{:?}", err);
            }
        }
    }

    pub fn store_data(&self) {
        if let Err(e) = store_data::<Schedule>(self.clone(), &SCHE_FILE_NAME.to_string()) {
            error!("{:?}", e);
        }
    }

    pub fn add_sche(&mut self, sche: ScheItem) {
        if self.schedules.len() <= 0 {
            self.load_data();
        }
        self.schedules.push(sche);
        self.store_data();
    }

    pub fn get_sches(&mut self) -> Vec<ScheItem> {
        self.load_data();
        self.schedules.clone()
    }
}

pub struct ScheUtilServer;

impl InternalServerInfo for ScheUtilServer {
    fn get_server(&self) -> super::internal_server::InternalMCPServer {
        get_server(
            "ScheduleUtil",
            "操作用户日程",
            vec![Box::new(ScheUtilAddSche), Box::new(ScheUtilGetSche)],
        )
    }
}
pub struct ScheUtilAddSche;

#[async_trait]
impl InternalFunctionCall for ScheUtilAddSche {
    async fn call(&self, args: Option<JsonObject>) -> Result<CallToolResult, String> {
        debug!("AddSche {:?}", args);
        if let Some(js) = args {
            if !js.contains_key("title")
                || !js.contains_key("content")
                || !js.contains_key("target_time")
            {
                return Err("AddSche 参数不完整".into());
            }
            let title = js.get("title").unwrap().as_str().unwrap().to_string();
            let content = js.get("content").unwrap().as_str().unwrap().to_string();
            let target_time_str = js.get("target_time").unwrap().as_str().unwrap();
            // 2015-09-05 23:56:04
            let target_time = NaiveDateTime::parse_from_str(target_time_str, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_local_timezone(Local)
                .unwrap();
            let data = ScheItem {
                title,
                content,
                status: EScheStatus::Unfinish,
                target_time,
                create_time: Local::now(),
                finish_time: Local::now(),
            };
            let mut sche = SCHEDULE.lock().await;
            sche.add_sche(data);
            return Ok(CallToolResult {
                content: vec![Annotated::new(
                    RawContent::Text(RawTextContent {
                        text: "成功".into(),
                    }),
                    None,
                )],
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
                },
                "target_time": {
                    "description":"日程日期，格式：2015-09-05 23:56:04",
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

pub struct ScheUtilGetSche;

#[async_trait]
impl InternalFunctionCall for ScheUtilGetSche {
    async fn call(&self, _args: Option<JsonObject>) -> Result<CallToolResult, String> {
        let mut sche = SCHEDULE.lock().await;
        let mut result = Vec::new();
        for item in sche.get_sches().iter() {
            result.push(Annotated::new(
                RawContent::Text(RawTextContent {
                    text: format!(
                        "标题: {}\n内容: {}\n状态: {:?}",
                        item.title, item.content, item.status
                    ),
                }),
                None,
            ));
        }
        Ok(CallToolResult {
            content: result,
            is_error: None,
        })
    }

    fn get_input_schema(&self) -> Arc<JsonObject> {
        Arc::new(
            serde_json::from_str(
                r#"
    {
        "properties":{}
    }"#,
            )
            .unwrap(),
        )
    }

    fn get_desc(&self) -> String {
        "获取所有日程安排".into()
    }

    fn get_name(&self) -> String {
        "get_sche".into()
    }
}
