use std::{sync::Arc};
use conversation::Conversation;
use lazy_static::lazy_static;
use log::{debug, warn};
use rolecard::RoleCard;
use tauri::AppHandle;
use think::{Think, ThinkConfig};
use tokio::sync::Mutex;

use crate::{data::{load_model_data, store_model_data, store_server_data, ServerData}, mcp::MCP_CLIENT, model::{Deepseek, EModelType, ModelData, ModelResponse, Ollama}};

mod conversation;
mod life;
mod think;
pub mod rolecard;

pub use rolecard::*;

#[derive(Debug)]
pub struct Assistant {
    conversation: Conversation,
    pub think: Think,
    pub server_data: ServerData,
}

lazy_static! {
    pub static ref ASSISTANT: Arc<Mutex<Assistant>> = Arc::new(Mutex::new(Assistant::new()));
    // 当次有效的 app handle
    pub static ref APP_HANDLE: Arc<Mutex<Option<AppHandle>>> = Arc::new(Mutex::new(None));
}

pub fn init() {
    debug!("初始化管家");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut ass: tokio::sync::MutexGuard<'_, Assistant> = ASSISTANT.lock().await;
        debug!("加载模型");
        ass.refresh_model_data();
        // debug!("初始化结束：{:?}", ass);
    });
}

impl Assistant {
    pub fn new()->Self {
        Self {
            conversation: Conversation::new(),
            think: Think::new(),
            server_data: ServerData::new(),
        }
    }

    pub async fn talk(&mut self, ctx: String, app: AppHandle)->Result<ModelResponse, String> {
        self.conversation.talk(ctx, app).await
    }

    pub async fn think_pulse(&mut self) {
        let res = self.conversation.system(self.think.get_think_string()).await;
        if res.is_err() {
            warn!("想法报错：{:?}", res);
            return;
        }
        let response = res.unwrap();
        debug!("想法 {:?} {}", response.reasoning_content, response.content);
        if let Some(tools) = response.tool_calls {
            for tool in tools.iter() {
                let mut client = MCP_CLIENT.lock().await;
                let ret= client.call_tool(tool.clone()).await;
                debug!("想法调用工具 {:?}", ret);
            }
        }
    }

    pub fn store_server_data(&self) {
        let _ = store_server_data(&self.server_data);
    }

    // 加载模型数据
    pub fn refresh_model_data(&mut self) {
        if let Ok(data) = load_model_data() {
            debug!("加载更新模型数据");
            self.conversation.set_model(data);
        }
        else {
            debug!("无配置，加载默认数据");
            self.conversation.set_model(ModelData::new(
                "https://api.deepseek.com/v1".into(),
            "deepseek-chat".into(),
            "".into(),
            EModelType::Deepseek));
        }
    }

    pub fn set_model_data(&mut self, data: ModelData) {
        self.conversation.set_model(data);
        self.store_model_data();
    }

    pub fn store_model_data(&self) {
        if let Err(e) = store_model_data(self.conversation.get_model_data().clone().unwrap_or_default()) {
            warn!("存储失败：{}", e);
        }
    }

    pub async fn get_models(&self)->Result<Vec<String>, String> {
        match self.conversation.get_model_data() {
            Some(model) => {
                match model.model_type {
                    EModelType::Deepseek |
                    EModelType::OpenAI => {
                        return Deepseek::get_models(&model).await;
                    },
                    EModelType::Ollama => {
                        return Ollama::get_models(&model).await;
                    },
                }
            },
            None => Err("未设置模型".into()),
        }
    }
}