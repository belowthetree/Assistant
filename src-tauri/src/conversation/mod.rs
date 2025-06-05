// mod step_talk;
mod talkcontext;

use crate::{
    event::ASSIST_REPLY_NAME,
    mcp::MCP_CLIENT,
    model::{ModelData, ModelInputParam, ModelMessage, ModelResponse, SseCallback},
};
use log::{debug, error, warn};
use rmcp::model::Tool;
use std::sync::{Arc, Mutex};
use talkcontext::{ERole, TalkContent, TalkContext};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Default)]
pub struct Conversation {
    model_data: Option<ModelData>,
    context: TalkContext,
    system: TalkContent,
    tools: Vec<Tool>,
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
                name: None,
                tool_call_id: None,
            },
            tools: Vec::new(),
            // step: StepTalkInfo::new(),
        }
    }

    pub fn set_model(&mut self, data: ModelData) {
        self.model_data = Some(data);
    }

    pub fn set_tools(&mut self, tools: Vec<Tool>) {
        self.tools = tools;
    }

    pub async fn talk(&mut self, ctx: String, app: AppHandle) -> Result<ModelResponse, String> {
        debug!("对话：{}", ctx);
        if self.model_data.is_none() {
            error!("未设置模型");
            return Err("未设置模型".into());
        }
        // 会话中存储用户输入
        self.context.add_user(ctx);
        // 流式回调
        let app = Arc::new(Mutex::new(app));
        let handle: SseCallback = Box::new(move |data| {
            // debug!("流式响应：{}", data);
            if let Ok(app) = app.lock() {
                if let Err(e) = app.emit(ASSIST_REPLY_NAME, data) {
                    warn!("emit: {:?}", e);
                }
            }
        });
        let res = self
            .send_to_model(
                ModelInputParam {
                    content: None,
                    system: None,
                    temperature: None,
                    tools: Some(self.tools.clone()),
                    messages: Some(self.context.get_messages(Some(&self.system))),
                },
                Some(handle),
            )
            .await;
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
                name: None,
                tool_call_id: None,
            });
        }
        return res;
    }

    pub async fn think(&mut self, ctx: String) -> Result<ModelResponse, String> {
        debug!("想法：{}", ctx);
        if self.model_data.is_none() {
            error!("未设置模型");
            return Err("未设置模型".into());
        }
        self.model_data.as_mut().unwrap().stream = false;
        // 会话中存储用户输入
        self.context.add_user(ctx);
        let res = self
            .send_to_model(
                ModelInputParam {
                    content: None,
                    system: None,
                    temperature: None,
                    tools: Some(self.tools.clone()),
                    messages: Some(self.context.get_messages(Some(&self.system))),
                },
                None,
            )
            .await;
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
                name: None,
                tool_call_id: None,
            });
        }
        return res;
    }

    #[allow(unused)]
    pub async fn system(&mut self, ctx: String) -> Result<ModelResponse, String> {
        debug!("系统：{}", ctx);
        let mut client = MCP_CLIENT.lock().await;
        let tools = client.get_all_tools().await.unwrap();
        if self.model_data.is_none() {
            error!("未设置模型");
            return Err("未设置模型".into());
        }
        // 存储系统输入
        self.add_system_context(ctx.clone());
        let res = self
            .send_to_model(
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
                name: None,
                tool_call_id: None,
            });
        }
        res
    }

    pub async fn tool(
        &mut self,
        tool_response: &Vec<ModelMessage>,
    ) -> Result<ModelResponse, String> {
        debug!("工具：{:?}", tool_response);
        if self.model_data.is_none() {
            error!("未设置模型");
            return Err("未设置模型".into());
        }
        // 存储工具输入
        for res in tool_response.iter() {
            self.add_tool_context(
                res.content.clone(),
                res.name.clone(),
                res.tool_call_id.clone(),
            );
        }
        let res = self
            .send_to_model(
                ModelInputParam {
                    content: None,
                    system: None,
                    temperature: None,
                    tools: Some(self.tools.clone()),
                    messages: Some(self.context.get_messages(None)),
                },
                None,
            )
            .await;
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
                name: None,
                tool_call_id: None,
            });
        }
        res
    }

    pub async fn send_to_model(
        &mut self,
        param: ModelInputParam,
        stream_callback: Option<SseCallback>,
    ) -> Result<ModelResponse, String> {
        let model = self.model_data.as_mut().unwrap();
        let res;
        debug!("send to model {:?}", param);
        match model.model_type {
            crate::model::EModelType::Deepseek | crate::model::EModelType::OpenAI => {
                res = crate::model::Deepseek::generate(model, param, stream_callback).await;
                debug!("{:?}", res);
            }
            crate::model::EModelType::Ollama => {
                res = crate::model::Ollama::generate(model, param).await;
            }
        }
        res
    }

    pub fn get_model_data(&self) -> Option<ModelData> {
        self.model_data.clone()
    }

    pub fn add_system_context(&mut self, ctx: String) {
        self.context.add_system(ctx);
    }

    pub fn add_tool_context(&mut self, ctx: String, name: String, tool_call_id: String) {
        self.context.add_content(&TalkContent {
            role: ERole::Tool,
            content: ctx,
            reasoning_content: None,
            tool_calls: None,
            name: Some(name),
            tool_call_id: Some(tool_call_id),
        });
    }

    // 设置系统级提示，与 add_system_context 不同，系统级提示每次都会添加在对话的最开头
    pub fn set_system(&mut self, ctx: String) {
        self.system = TalkContent {
            role: ERole::System,
            content: ctx,
            reasoning_content: None,
            tool_calls: None,
            name: None,
            tool_call_id: None,
        }
    }

    pub fn clear_context(&mut self) {
        self.context.clear();
    }
}
