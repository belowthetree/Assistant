use conversation::Conversation;
use lazy_static::lazy_static;
use log::{debug, warn};
use std::{collections::HashMap, sync::Arc};
use tauri::AppHandle;
use think::Think;
use tokio::sync::Mutex;

use crate::{
    data::{
        load_model_data, load_rolecard_data, store_model_data, store_rolecard_data,
        store_server_data, ServerData,
    },
    mcp::MCP_CLIENT,
    model::{Deepseek, EModelType, ModelData, ModelResponse, Ollama, ToolCall},
};

mod conversation;
mod life;
pub mod rolecard;
mod think;

pub use rolecard::*;

#[derive(Debug)]
pub struct Assistant {
    user_conversation: Conversation,
    think_conversation: Conversation,
    pub think: Think,
    pub server_data: ServerData,
    max_count: usize,
    count: usize,
    max_think_count: usize,
    think_count: usize,
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
        debug!("加载角色卡");
        ass.refresh_rolecard();
        // debug!("初始化结束：{:?}", ass);
    });
}

impl Assistant {
    pub fn new() -> Self {
        Self {
            user_conversation: Conversation::new(),
            think_conversation: Conversation::new(),
            think: Think::new(),
            server_data: ServerData::new(),
            max_count: 6,
            count: 0,
            max_think_count: 5,
            think_count: 0,
        }
    }

    pub async fn talk(&mut self, ctx: String, app: AppHandle) -> Result<ModelResponse, String> {
        if self.count >= self.max_count {
            return Err("达到最大循环次数".into());
        }
        self.count += 1;
        let mut response = self.user_conversation.talk(ctx, app.clone()).await;
        if response.is_ok() {
            let res = response.clone().unwrap();
            let message = self.process_tool_call(&res.tool_calls).await;
            // 开始循环返回工具调用信息给模型
            if message.len() > 0 {
                response = Box::pin(self.system(serde_json::to_string(&message).unwrap())).await;
            }
        }
        self.count -= 1;
        response
    }

    // 工具的回复以系统级反馈回去
    async fn system(&mut self, ctx: String) -> Result<ModelResponse, String> {
        if self.count >= self.max_count {
            return Err("达到最大循环次数".into());
        }
        self.count += 1;
        let mut response = self.user_conversation.system(ctx).await;
        if response.is_ok() {
            let res = response.clone().unwrap();
            let message = self.process_tool_call(&res.tool_calls).await;
            // 开始循环返回工具调用信息给模型
            if message.len() > 0 {
                response = Box::pin(self.system(serde_json::to_string(&message).unwrap())).await;
            }
        }
        self.count -= 1;
        response
    }

    // 处理工具调用，返回工具名、结果的键值对
    async fn process_tool_call(
        &mut self,
        tools: &Option<Vec<ToolCall>>,
    ) -> HashMap<String, String> {
        let mut message = HashMap::new();
        if let Some(tools) = tools {
            let mut client = MCP_CLIENT.lock().await;
            for tool in tools.iter() {
                let ret = client.call_tool(tool.clone()).await;
                debug!("assistant tool call {:?}", ret);
                message.insert(
                    tool.function.name.clone(),
                    serde_json::to_string(&ret).unwrap(),
                );
            }
        }
        debug!("system 工具调用：{:?}", message);
        message
    }

    // 唤醒后触发思考
    pub async fn think_pulse(&mut self) {
        let res = self
            .think_conversation
            .system(self.think.get_think_string().await)
            .await;
        if res.is_err() {
            warn!("想法报错：{:?}", res);
            return;
        }
        let response = res.unwrap();
        debug!("想法 {:?} {}", response.reasoning_content, response.content);
        debug!("最终结果：{:?}", self.think_recycle(Ok(response)).await);
    }

    async fn think_recycle(
        &mut self,
        response: Result<ModelResponse, String>,
    ) -> Result<(), String> {
        if response.is_err() {
            return Err(response.unwrap_err());
        }
        if self.think_count >= self.max_think_count {
            warn!("想法达到最大循环次数");
            return Err("想法达到最大循环次数".into());
        }
        let mut message = HashMap::new();
        let res = response.unwrap();
        if res.tool_calls.is_none() {
            return Ok(());
        }
        if let Some(tools) = res.tool_calls {
            for tool in tools.iter() {
                let mut client = MCP_CLIENT.lock().await;
                let ret = client.call_tool(tool.clone()).await;
                debug!("想法调用工具 {:?}", ret);
                message.insert(
                    tool.function.name.clone(),
                    serde_json::to_string(&ret).unwrap(),
                );
            }
        }
        let mut response = Err("调用无结果".into());
        self.think_count += 1;
        if message.len() > 0 {
            let res = self
                .think_conversation
                .system(serde_json::to_string(&message).unwrap())
                .await;
            response = Box::pin(self.think_recycle(res)).await;
        }
        self.think_count -= 1;
        response
    }

    pub fn store_server_data(&self) {
        let _ = store_server_data(&self.server_data);
    }

    // 加载模型数据
    pub fn refresh_model_data(&mut self) {
        if let Ok(data) = load_model_data() {
            debug!("加载更新模型数据 {:?}", data);
            self.user_conversation.set_model(data.clone());
            self.think_conversation.set_model(data);
        } else {
            debug!("无配置，加载默认数据");
            self.user_conversation.set_model(ModelData::new(
                "https://api.deepseek.com/v1".into(),
                "deepseek-chat".into(),
                "".into(),
                EModelType::Deepseek,
            ));
            self.think_conversation.set_model(ModelData::new(
                "https://api.deepseek.com/v1".into(),
                "deepseek-chat".into(),
                "".into(),
                EModelType::Deepseek,
            ));
        }
    }

    pub fn set_model_data(&mut self, data: ModelData) {
        self.user_conversation.set_model(data);
        self.store_model_data();
    }

    pub fn store_model_data(&self) {
        if let Err(e) = store_model_data(
            self.user_conversation
                .get_model_data()
                .clone()
                .unwrap_or_default(),
        ) {
            warn!("存储失败：{}", e);
        }
    }

    pub async fn get_models(&self) -> Result<Vec<String>, String> {
        match self.user_conversation.get_model_data() {
            Some(model) => match model.model_type {
                EModelType::Deepseek | EModelType::OpenAI => {
                    return Deepseek::get_models(&model).await;
                }
                EModelType::Ollama => {
                    return Ollama::get_models(&model).await;
                }
            },
            None => Err("未设置模型".into()),
        }
    }

    // 更新角色卡数据（也就是 system 提示）
    pub fn refresh_rolecard(&mut self) {
        let ret = load_rolecard_data();
        if let Ok(mut data) = ret {
            // 生成默认助手配置
            if !data.cards.contains_key(ASSISTANT_NAME) {
                data.cards
                    .insert(ASSISTANT_NAME.to_string(), self.think.rolecard.clone());
                // 写回配置
                let ret = store_rolecard_data(&data);
                if ret.is_err() {
                    warn!("refresh_rolecard {:?}", ret);
                }
            }
            // 获取当前配置的助手信息
            let mut assistant_role = data.assistant_role.clone();
            if !data.cards.contains_key(&assistant_role) {
                assistant_role = ASSISTANT_NAME.to_string();
                data.assistant_role = assistant_role.clone();
                // 写回配置
                let ret = store_rolecard_data(&data);
                if ret.is_err() {
                    warn!("refresh_rolecard {:?}", ret);
                }
            }
            self.think
                .set_rolecard(data.cards.get(&assistant_role).unwrap().clone());
            self.user_conversation
                .set_system(self.think.get_conversation_string());
            return;
        }
        // 如果读取失败，可能是因为没有存档，需要生成
        let mut data = RoleCardStoreData {
            assistant_role: ASSISTANT_NAME.to_string(),
            cards: HashMap::new(),
        };
        data.cards
            .insert(ASSISTANT_NAME.to_string(), self.think.rolecard.clone());
        self.think
            .set_rolecard(data.cards.get(ASSISTANT_NAME).unwrap().clone());
        self.user_conversation
            .set_system(self.think.get_conversation_string());
        let ret = store_rolecard_data(&data);
        if ret.is_err() {
            warn!("refresh_rolecard {:?}", ret);
        }
    }
}
