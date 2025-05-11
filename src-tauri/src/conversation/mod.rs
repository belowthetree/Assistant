mod talkcontext;

use std::sync::Arc;

use lazy_static::lazy_static;
use log::debug;
use talkcontext::TalkContext;
use tokio::sync::Mutex;
use crate::{data::load_model_data, model::{EModelType, ModelData, ModelInputParam, ModelInteract, ModelResponse}};

lazy_static! {
    static ref CONVERSATION: Arc<Mutex<Conversation>> = Arc::new(Mutex::new(Conversation::new()));
}

pub fn init() {
    debug!("初始化基础对话");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let data: ModelData;
        if let Ok(d) = load_model_data().await {
            data = d;
        }
        else {
            data = ModelData::new(
                "https://api.deepseek.com/v1".into(),
            "deepseek-chat".into(),
            "sk-25c8486584eb474bb3d175a61d503201".into(),
            EModelType::Deepseek);
        }
        let mut con = CONVERSATION.lock().await;
        con.set_model(data);
    })
}

#[tauri::command]
pub async fn talk(ctx: String)->Result<String, String> {
    let mut con = CONVERSATION.lock().await;
    let res = con.talk(ctx).await;
    if res.is_ok() {
        Ok(res.unwrap().content)
    }
    else {
        Err(res.unwrap_err())
    }
}

pub struct Conversation {
    model_data: Option<ModelData>,
    context: TalkContext,
}

impl Conversation {
    pub fn new()->Self {
        Self {
            model_data: None,
            context: TalkContext::new(),
        }
    }

    pub fn set_model(&mut self, data: ModelData) {
        let key = data.api_key.clone();
        self.model_data = Some(data);
        let d = self.model_data.as_mut().unwrap();
        d.set_api_key(key);
    }

    pub async fn talk(&mut self, ctx: String)->Result<ModelResponse, String> {
        if let Some(model) = &self.model_data {
            match model.model_type {
                crate::model::EModelType::Deepseek | crate::model::EModelType::OpenAI => {
                    return crate::model::Deepseek::generate(model, ModelInputParam {
                        content: None,
                        system: None,
                        temperature: None,
                        tools: None,
                        messages: Some(self.context.get_messages()),
                    }).await;
                },
                crate::model::EModelType::Ollama => {
                    return Err("暂未实现 Ollama".into());
                },
            }
        }
        Err("未设置模型数据".into())
    }
}