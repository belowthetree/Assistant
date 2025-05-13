mod talkcontext;

use talkcontext::{ERole, TalkContext};
use crate::{model::{ModelData, ModelInputParam, ModelResponse}};

#[derive(Debug, Clone)]
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
        self.model_data = Some(data);
    }

    pub async fn talk(&mut self, ctx: String)->Result<ModelResponse, String> {
        if let Some(model) = &self.model_data {
            // 会话中存储用户输入
            self.context.add_user(ctx);
            let res;
            match model.model_type {
                crate::model::EModelType::Deepseek | crate::model::EModelType::OpenAI => {
                    res = crate::model::Deepseek::generate(model, ModelInputParam {
                        content: None,
                        system: None,
                        temperature: None,
                        tools: None,
                        messages: Some(self.context.get_messages()),
                    }, None).await;
                },
                crate::model::EModelType::Ollama => {
                    res = crate::model::Ollama::generate(model, ModelInputParam {
                        content: None,
                        system: None,
                        temperature: None,
                        tools: None,
                        messages: Some(self.context.get_messages()),
                    }).await;
                },
            }
            let tmp = res.clone();
            // 模型的答复写回会话中
            if let Ok(response) = tmp {
                self.context.add_assistant(&response);
            }
            else {
                self.context.add_content(&talkcontext::TalkContent {
                    role: ERole::Assistant,
                    content: tmp.unwrap_err(),
                    reasoning_content: None,
                    tool_calls: None,
                });
            }
            return res;
        }
        Err("未设置模型数据".into())
    }

    pub fn get_model_data(&self)->Option<ModelData> {
        self.model_data.clone()
    }
}