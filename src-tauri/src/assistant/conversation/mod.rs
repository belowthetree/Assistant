mod step_talk;
mod talkcontext;

use crate::{
    event::ASSIST_REPLY_NAME,
    mcp::MCP_CLIENT,
    model::{ModelData, ModelInputParam, ModelResponse, SseCallback},
};
use log::{debug, error, warn};
use std::sync::{Arc, Mutex};
use step_talk::StepTalkInfo;
use talkcontext::{ERole, TalkContent, TalkContext};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone)]
pub struct Conversation {
    model_data: Option<ModelData>,
    context: TalkContext,
    system: TalkContent,
    // step: StepTalkInfo,
}

impl Conversation {
    pub fn new() -> Self {
        Self {
            model_data: None,
            context: TalkContext::new(),
            system: TalkContent {
                role: ERole::System,
                content: "".into(),
                reasoning_content: None,
                tool_calls: None,
            },
            // step: StepTalkInfo::new(),
        }
    }

    pub fn set_model(&mut self, data: ModelData) {
        self.model_data = Some(data);
    }

    pub async fn talk(&mut self, ctx: String, app: AppHandle) -> Result<ModelResponse, String> {
        debug!("对话：{}", ctx);
        let mut client = MCP_CLIENT.lock().await;
        let tools = client.get_all_tools().await.unwrap();
        if self.model_data.is_none() {
            error!("未设置模型");
            return Err("未设置模型".into());
        }
        let model = self.model_data.as_mut().unwrap();
        // 会话中存储用户输入
        self.context.add_user(ctx);
        let res;
        match model.model_type {
            crate::model::EModelType::Deepseek | crate::model::EModelType::OpenAI => {
                let app = Arc::new(Mutex::new(app));
                let handle: SseCallback = Box::new(move |data| {
                    // debug!("流式响应：{}", data);
                    if let Ok(app) = app.lock() {
                        if let Err(e) = app.emit(ASSIST_REPLY_NAME, data) {
                            warn!("emit: {:?}", e);
                        }
                    }
                });
                res = crate::model::Deepseek::generate(
                    model,
                    ModelInputParam {
                        content: None,
                        system: None,
                        temperature: None,
                        tools: Some(tools),
                        messages: Some(self.context.get_messages(Some(&self.system))),
                    },
                    Some(handle),
                )
                .await;
                debug!("{:?}", res);
            }
            crate::model::EModelType::Ollama => {
                res = crate::model::Ollama::generate(
                    model,
                    ModelInputParam {
                        content: None,
                        system: None,
                        temperature: None,
                        tools: Some(tools),
                        messages: Some(self.context.get_messages(Some(&self.system))),
                    },
                )
                .await;
            }
        }
        let tmp = res.clone();
        // 模型的答复写回会话中
        if let Ok(response) = tmp {
            self.context.add_assistant(&response);
        } else {
            self.context.add_content(&talkcontext::TalkContent {
                role: ERole::Assistant,
                content: tmp.unwrap_err(),
                reasoning_content: None,
                tool_calls: None,
            });
        }
        return res;
    }

    pub async fn system(&mut self, ctx: String) -> Result<ModelResponse, String> {
        debug!("系统：{}", ctx);
        let mut client = MCP_CLIENT.lock().await;
        let tools = client.get_all_tools().await.unwrap();
        if self.model_data.is_none() {
            error!("未设置模型");
            return Err("未设置模型".into());
        }
        let res;
        let model = self.model_data.as_mut().unwrap();
        match model.model_type {
            crate::model::EModelType::Deepseek | crate::model::EModelType::OpenAI => {
                res = crate::model::Deepseek::generate(
                    model,
                    ModelInputParam {
                        content: None,
                        system: Some(ctx),
                        temperature: None,
                        tools: Some(tools),
                        messages: None,
                    },
                    None,
                )
                .await;
                debug!("{:?}", res);
            }
            crate::model::EModelType::Ollama => {
                res = crate::model::Ollama::generate(
                    model,
                    ModelInputParam {
                        content: None,
                        system: None,
                        temperature: None,
                        tools: Some(tools),
                        messages: Some(self.context.get_messages(Some(&self.system))),
                    },
                )
                .await;
            }
        }
        return res;
    }

    pub fn get_model_data(&self) -> Option<ModelData> {
        self.model_data.clone()
    }

    pub fn add_system_context(&mut self, ctx: String) {
        self.context.add_system(ctx);
    }

    // 设置系统级提示，与 add_system_context 不同，系统级提示每次都会添加在对话的最开头
    pub fn set_system(&mut self, ctx: String) {
        self.system = TalkContent {
            role: ERole::System,
            content: ctx,
            reasoning_content: None,
            tool_calls: None,
        }
    }
}
