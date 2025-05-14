use std::{sync::Arc};

use conversation::Conversation;
use life::Life;
use lazy_static::lazy_static;
use log::{debug, warn};
use tauri::AppHandle;
use tokio::sync::Mutex;

use crate::{data::{load_model_data, load_server_data, store_model_data, store_server_data, ServerData}, model::{Deepseek, EModelType, ModelData, ModelResponse, Ollama}};

mod conversation;
mod life;
mod think;

#[derive(Debug, Clone)]
pub struct Assistant {
    conversation: Conversation,
    pub life: Life,
    pub server_data: ServerData,
}

lazy_static! {
    pub static ref ASSISTANT: Arc<Mutex<Assistant>> = Arc::new(Mutex::new(Assistant::new()));
}

pub fn init() {
    debug!("初始化管家");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut ass: tokio::sync::MutexGuard<'_, Assistant> = ASSISTANT.lock().await;
        debug!("加载模型");
        ass.refresh_model_data();
        debug!("初始化结束：{:?}", ass);
    });
}

#[tauri::command]
pub async fn talk(ctx: String, app: AppHandle)->Result<String, String> {
    let mut con = ASSISTANT.lock().await;
    let res = con.talk(ctx, app).await;
    if res.is_ok() {
        Ok(res.unwrap().content)
    }
    else {
        Err(res.unwrap_err())
    }
}

#[tauri::command]
pub async fn update_model() {
    let mut ass: tokio::sync::MutexGuard<'_, Assistant> = ASSISTANT.lock().await;
    ass.refresh_model_data();
}

impl Assistant {
    pub fn new()->Self {
        Self {
            conversation: Conversation::new(),
            life: Life::new(),
            server_data: ServerData::new(),
        }
    }

    pub async fn talk(&mut self, ctx: String, app: AppHandle)->Result<ModelResponse, String> {
        self.conversation.talk(ctx, app).await
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