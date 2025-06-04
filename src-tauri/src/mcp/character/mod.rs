use async_trait::async_trait;
use lazy_static::lazy_static;
use log::{debug, warn};
use rmcp::model::{Annotated, CallToolResult, JsonObject, RawContent, RawTextContent};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    assistant::{RoleCard, ASSISTANT_NAME},
    conversation::Conversation,
    data::{
        load_data, load_model_data, load_rolecard_data, read_role_directory, store_data,
        store_rolecard_data, store_to_role_directory,
    },
    mcp::{get_internal_servers, MCPClient, MCP_CLIENT},
    model::{ModelMessage, ModelResponse, ToolCall},
};

use super::{
    get_server,
    internal_server::{InternalFunctionCall, InternalMCPServer},
    InternalServerInfo,
};

lazy_static! {
    static ref CHARACTERS: Arc<Mutex<Vec<Character>>> = Arc::new(Mutex::new(Vec::new()));
    static ref CHARACTER_WRITEBACK: Mutex<usize> = Mutex::new(1);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterTask {
    pub target: String,
    #[serde(default)]
    pub result: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterWorkingInfo {
    pub target: String,
    pub tasks: Vec<CharacterTask>,
    pub is_working: bool,
    pub current_task_index: usize,
}

/// Character
/// 对 Character 本身数据更改没有意义，这个只是运行时备份，实际数据需要从文件中读写
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Character {
    // 角色卡对应的名字
    pub rolename: String,
    pub working: CharacterWorkingInfo,
    #[serde(skip_serializing, skip_deserializing)]
    pub rolecard: RoleCard,
    #[serde(skip_serializing, skip_deserializing)]
    pub conversation: Conversation,
    #[serde(skip_serializing, skip_deserializing)]
    pub max_think_count: usize,
    #[serde(skip_serializing, skip_deserializing)]
    pub think_count: usize,
}

async fn write_character(character: Character) -> Result<(), String> {
    let _ = CHARACTER_WRITEBACK.lock().await;
    let name = character.rolename.clone() + ".json";
    store_data(character, &name)
}

async fn read_character(name: &str) -> Result<Character, String> {
    let _ = CHARACTER_WRITEBACK.lock().await;
    load_data::<Character>(&(name.to_string() + ".json"))
}

impl Character {
    fn new(rolecard: RoleCard) -> Self {
        let mut conversation = Conversation::new();
        if let Ok(mut data) = load_model_data() {
            debug!("加载角色模型数据 {:?}", data);
            data.stream = false;
            conversation.set_model(data);
            conversation.set_system(rolecard.get_prompt());
        } else {
            warn!("角色模型加载失败");
        }
        Self {
            rolename: rolecard.name.clone(),
            working: CharacterWorkingInfo {
                target: "".into(),
                tasks: Vec::new(),
                is_working: false,
                current_task_index: 0,
            },
            rolecard,
            conversation,
            max_think_count: 10,
            think_count: 0,
        }
    }

    pub async fn load(name: &str) -> Result<Self, String> {
        let _ = CHARACTER_WRITEBACK.lock().await;
        let mut chs = CHARACTERS.lock().await;
        for ch in chs.iter() {
            if ch.rolename == name {
                return Ok(ch.clone());
            }
        }
        // 先加载角色卡
        let data = load_rolecard_data();
        if data.is_err() {
            warn!("加载人物角色卡失败 {:?}", data);
            return Err(format!("加载人物角色卡失败 {:?}", data));
        }
        let data = data.unwrap();
        // 角色卡不存在视为人物不存在
        if !data.cards.contains_key(name) {
            warn!("不存在此人物 {}", name);
            return Err(format!("不存在此人物 {}", name));
        }
        let rolecard = data.cards[name].clone();
        // 加载角色数据
        let data = read_character(name).await;
        // 不存在数据则新建一个
        if data.is_err() {
            chs.push(Self::new(rolecard));
            Ok(chs.last().unwrap().clone())
        } else {
            // 存在则继续沿用
            let mut character = data.unwrap();
            character.rolecard = rolecard;
            chs.push(character);
            Ok(chs.last().unwrap().clone())
        }
    }

    pub async fn set_target(&mut self, target: &str) -> Result<(), String> {
        debug!("设置角色目标 {:?}", target);
        self.working.target = target.to_string();
        self.conversation.clear_context();
        let res = self
            .conversation
            .think(target.to_string() + "请认真思考，然后通过提供的函数设置好各阶段的任务")
            .await;
        if res.is_err() {
            return Err(res.unwrap_err());
        }
        let mut client = MCPClient::new();
        client
            .set_internal_servers(get_internal_servers(vec![Box::new(CharacterSelfServer {
                name: self.rolename.clone(),
            })]))
            .await;
        let res = res.unwrap();
        if let Some(tools) = res.tool_calls {
            debug!(
                "人物 {} 设置目标结果 {:?}",
                self.rolename.clone(),
                self.call_tools(tools, Some(&mut client)).await
            );
            let res = self.talk("现在开始你的工作".into()).await;
            if res.is_err() {
                Err(res.unwrap_err())
            } else {
                Ok(())
            }
        } else {
            Err("没有调用任务设置".into())
        }
    }

    pub async fn talk(&mut self, ctx: String) -> Result<String, String> {
        let mut client = MCPClient::new();
        client
            .set_internal_servers(get_internal_servers(vec![Box::new(CharacterSelfServer {
                name: self.rolename.clone(),
            })]))
            .await;
        client.refresh_mcp_config().await;
        let tools = client.get_all_tools().await;
        if tools.is_err() {
            return Err(tools.unwrap_err());
        }
        self.conversation.set_tools(tools.unwrap());
        let res = self.conversation.think(ctx).await;
        if res.is_err() {
            return Err(res.unwrap_err());
        }
        let res = self.recycle_think(res).await;
        if res.is_err() {
            Err(res.unwrap_err())
        } else {
            Ok(res.unwrap().content)
        }
    }

    pub async fn recycle_think(
        &mut self,
        response: Result<ModelResponse, String>,
    ) -> Result<ModelResponse, String> {
        if response.is_err() {
            return Err(response.unwrap_err());
        }
        let res = response.unwrap();
        if res.tool_calls.is_none() {
            return Ok(res);
        }
        if self.think_count >= self.max_think_count {
            return Err("达到最大循环调用思考次数".into());
        }
        let mut message = Vec::new();
        if let Some(tools) = res.tool_calls {
            message = self.call_tools(tools, None).await;
        }
        let mut response = Err("调用人物结果".into());
        self.think_count += 1;
        if message.len() > 0 {
            let res = self.conversation.tool(&message).await;
            response = Box::pin(self.recycle_think(res)).await;
        }
        self.think_count -= 1;
        response
    }

    pub async fn call_tools(
        &self,
        tools: Vec<ToolCall>,
        mut client: Option<&mut MCPClient>,
    ) -> Vec<ModelMessage> {
        let mut message = Vec::new();
        for tool in tools.iter() {
            let ret;
            if let Some(ref mut client) = client {
                ret = client.call_tool(tool.clone()).await;
            } else {
                let mut client = MCP_CLIENT.lock().await;
                ret = client.call_tool(tool.clone()).await;
            }
            debug!("人物 {} 调用工具 {:?}", self.rolename.clone(), ret);
            message.push(ModelMessage {
                role: "tool".into(),
                content: serde_json::to_string(&ret).unwrap(),
                name: tool.function.name.clone(),
                tool_call_id: tool.id.clone(),
                tool_calls: None,
            });
        }
        message
    }
}

pub struct CharacterOutServer;

pub struct CharacterSelfServer {
    pub name: String,
}

impl InternalServerInfo for CharacterSelfServer {
    fn get_server(&self) -> InternalMCPServer {
        get_server(
            "CharacterServer",
            "人物角色操作，获取工作信息，对工作空间的文件进行读写",
            vec![
                Box::new(CharacterGetSelfWorkingInfo {
                    name: self.name.clone(),
                }),
                Box::new(CharacterSetSelfTasks {
                    name: self.name.clone(),
                }),
                Box::new(CharacterWriteFile {
                    name: self.name.clone(),
                }),
                Box::new(CharacterReadFile {
                    name: self.name.clone(),
                }),
            ],
        )
    }
}

impl InternalServerInfo for CharacterOutServer {
    fn get_server(&self) -> InternalMCPServer {
        get_server(
            "CharacterServer",
            "人物角色操作，可以获取人物信息，给不同角色分配任务",
            vec![
                Box::new(CharacterGetAvailableCharacter),
                Box::new(CharacterGetCharacterWorkingInfo),
                Box::new(CharacterSetTargetAndWork),
                Box::new(CreateCharacter),
                Box::new(DeleteCharacter),
            ],
        )
    }
}

/// 创建新角色
pub struct CreateCharacter;

#[async_trait]
impl InternalFunctionCall for CreateCharacter {
    async fn call(&self, arg: Option<JsonObject>) -> Result<CallToolResult, String> {
        debug!("CreateCharacter {:?}", arg);
        if arg.is_none() {
            return Err("缺少参数".into());
        }
        let arg = arg.unwrap();
        if !arg.contains_key("name") {
            return Err("缺少参数 name".into());
        }
        if !arg.contains_key("desc") {
            return Err("缺少参数 desc".into());
        }
        if !arg.contains_key("conversation_prompt") {
            return Err("缺少参数 conversation_prompt".into());
        }
        let name = arg.get("name").unwrap().as_str();
        if name.is_none() {
            return Err("参数 name 格式错误".into());
        }
        let name = name.unwrap();

        // 创建新角色数据
        let mut rolecard = RoleCard {
            name: name.to_string(),
            ..Default::default()
        };

        // 设置角色卡属性（如果有参数）
        if let Some(conversation_prompt) = arg.get("conversation_prompt").and_then(|v| v.as_str()) {
            rolecard.conversation_prompt = conversation_prompt.to_string();
        }
        if let Some(desc) = arg.get("desc").and_then(|v| v.as_str()) {
            rolecard.desc = desc.to_string();
        }

        // 保存角色卡
        let mut data = load_rolecard_data().unwrap_or_default();
        data.cards.insert(name.to_string(), rolecard);

        if let Err(e) = store_rolecard_data(&data) {
            return Err(format!("保存角色卡失败: {}", e));
        }

        Ok(CallToolResult {
            content: vec![Annotated::new(
                RawContent::Text(RawTextContent {
                    text: serde_json::to_string("角色创建成功").unwrap(),
                }),
                None,
            )],
            is_error: None,
        })
    }

    fn get_input_schema(&self) -> Arc<JsonObject> {
        Arc::new(
            serde_json::from_str(
                r#"
        {
            "properties":{
                "name": {
                    "description":"角色名字",
                    "type": "string"
                },
                "conversation_prompt": {
                    "description":"角色对话提示语",
                    "type": "string"
                },
                "desc": {
                    "description":"角色描述",
                    "type": "string"
                }
            }
        }"#,
            )
            .unwrap(),
        )
    }

    fn get_desc(&self) -> String {
        "创建新角色".into()
    }

    fn get_name(&self) -> String {
        "create_character".into()
    }
}

/// 删除角色
pub struct DeleteCharacter;

#[async_trait]
impl InternalFunctionCall for DeleteCharacter {
    async fn call(&self, arg: Option<JsonObject>) -> Result<CallToolResult, String> {
        debug!("DeleteCharacter {:?}", arg);
        if arg.is_none() {
            return Err("缺少参数".into());
        }
        let arg = arg.unwrap();
        let name = arg.get("name").unwrap().as_str();
        if name.is_none() {
            return Err("参数 name 格式错误".into());
        }
        let name = name.unwrap();
        // 从角色卡数据中移除
        let mut data = load_rolecard_data().unwrap_or_default();
        if !data.cards.contains_key(name) {
            return Err("角色不存在".into());
        }
        data.cards.remove(name);

        if let Err(e) = store_rolecard_data(&data) {
            return Err(format!("更新角色卡数据失败: {}", e));
        }

        // 删除角色数据文件
        let filename = format!("{}.json", name);
        if let Err(e) = std::fs::remove_file(filename) {
            return Err(format!("删除角色数据文件失败: {}", e));
        }

        // 从内存中移除
        let mut characters = CHARACTERS.lock().await;
        characters.retain(|c| c.rolename != name);

        Ok(CallToolResult {
            content: vec![Annotated::new(
                RawContent::Text(RawTextContent {
                    text: serde_json::to_string("角色删除成功").unwrap(),
                }),
                None,
            )],
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
        "删除当前角色".into()
    }

    fn get_name(&self) -> String {
        "delete_character".into()
    }
}

/// 获取已有的角色
pub struct CharacterGetAvailableCharacter;

#[async_trait]
impl InternalFunctionCall for CharacterGetAvailableCharacter {
    async fn call(&self, _: Option<JsonObject>) -> Result<CallToolResult, String> {
        debug!("GetAvailableCharacter");
        let mut rolecards = load_rolecard_data().unwrap();
        rolecards.cards.remove(ASSISTANT_NAME);
        let mut content = Vec::new();
        for card in rolecards.cards.iter() {
            content.push(Annotated::new(
                RawContent::Text(RawTextContent {
                    text: serde_json::to_string(&card).unwrap(),
                }),
                None,
            ));
        }
        return Ok(CallToolResult {
            content,
            is_error: None,
        });
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
        "获取可以指挥的人物角色描述".into()
    }

    fn get_name(&self) -> String {
        "get_available_character".into()
    }
}

/// 获取指定角色信息
pub struct CharacterGetCharacterWorkingInfo;

#[async_trait]
impl InternalFunctionCall for CharacterGetCharacterWorkingInfo {
    async fn call(&self, arg: Option<JsonObject>) -> Result<CallToolResult, String> {
        debug!("GetCharacterWorkingInfo {:?}", arg);
        if !arg.is_none() {
            return Err("缺少参数".into());
        }
        let arg = arg.unwrap();
        if !arg.contains_key("name") {
            return Err("缺少参数 name".into());
        }
        let name = arg.get("name").unwrap().as_str().unwrap();
        let res = Character::load(name).await;
        if res.is_err() {
            return Err("不存在此角色数据".into());
        }
        let character = res.unwrap();
        return Ok(CallToolResult {
            content: vec![Annotated::new(
                RawContent::Text(RawTextContent {
                    text: serde_json::to_string(&character).unwrap(),
                }),
                None,
            )],
            is_error: None,
        });
    }

    fn get_input_schema(&self) -> Arc<JsonObject> {
        Arc::new(
            serde_json::from_str(
                r#"
        {
            "properties":{
                "name":{
                    "description":"角色名字",
                    "type":"string"
                }
            }
        }"#,
            )
            .unwrap(),
        )
    }

    fn get_desc(&self) -> String {
        "获取指定角色的工作信息".into()
    }

    fn get_name(&self) -> String {
        "get_character_working_info".into()
    }
}

/// 给角色设置工作目标并开始干活
pub struct CharacterSetTargetAndWork;

#[async_trait]
impl InternalFunctionCall for CharacterSetTargetAndWork {
    async fn call(&self, arg: Option<JsonObject>) -> Result<CallToolResult, String> {
        debug!("CharacterSetTarget {:?}", arg);
        if arg.is_none() {
            return Err("缺少参数".into());
        }
        let arg = arg.unwrap();
        if !arg.contains_key("name") {
            return Err("缺少参数 name".into());
        }
        if !arg.contains_key("target") {
            return Err("缺少参数 target".into());
        }
        let name = arg.get("name").unwrap().as_str().unwrap();
        let target = arg.get("target").unwrap().as_str().unwrap().to_string();
        let res = Character::load(name).await;
        if res.is_err() {
            return Err("不存在此角色数据".into());
        }
        let mut character = res.unwrap();
        tokio::spawn(async move {
            let res = character.set_target(&target).await;
            if res.is_err() {
                warn!("设置目标出错 {:?}", res);
            }
        });
        return Ok(CallToolResult {
            content: vec![Annotated::new(
                RawContent::Text(RawTextContent {
                    text: serde_json::to_string("成功").unwrap(),
                }),
                None,
            )],
            is_error: None,
        });
    }

    fn get_input_schema(&self) -> Arc<JsonObject> {
        Arc::new(
            serde_json::from_str(
                r#"
        {
            "properties":{
                "name":{
                    "description":"角色名字",
                    "type": "string"
                },
                "target": {
                    "description":"目标描述",
                    "type": "string"
                }
            }
        }"#,
            )
            .unwrap(),
        )
    }

    fn get_desc(&self) -> String {
        "给角色分配任务，这将会打断当前任务，从头开始执行新任务".into()
    }

    fn get_name(&self) -> String {
        "set_character_target_and_work".into()
    }
}

/// 与角色对话
pub struct CharacterTalk;

#[async_trait]
impl InternalFunctionCall for CharacterTalk {
    async fn call(&self, arg: Option<JsonObject>) -> Result<CallToolResult, String> {
        if arg.is_none() {
            return Err("没有参数".into());
        }
        let arg = arg.unwrap();
        if !arg.contains_key("name") {
            return Err("没有参数 name".into());
        }
        if !arg.contains_key("content") {
            return Err("没有参数 content".into());
        }
        let name = arg.get("name").unwrap().as_str();
        if name.is_none() {
            return Err("参数 name 格式错误".into());
        }
        let content = arg.get("content").unwrap().as_str();
        if content.is_none() {
            return Err("参数 content 格式错误".into());
        }
        let content = content.unwrap();
        let name = name.unwrap();
        let character = Character::load(name).await;
        if character.is_err() {
            return Err(character.unwrap_err());
        }
        let mut character = character.unwrap();
        let res = character.talk(content.to_string()).await;
        if res.is_ok() {
            Ok(CallToolResult {
                content: vec![Annotated::new(
                    RawContent::Text(RawTextContent {
                        text: serde_json::to_string("成功").unwrap(),
                    }),
                    None,
                )],
                is_error: None,
            })
        } else {
            Err(res.unwrap_err())
        }
    }

    fn get_input_schema(&self) -> Arc<JsonObject> {
        todo!()
    }

    fn get_desc(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }
}

/// 角色自身获取信息
pub struct CharacterGetSelfWorkingInfo {
    pub name: String,
}

#[async_trait]
impl InternalFunctionCall for CharacterGetSelfWorkingInfo {
    async fn call(&self, arg: Option<JsonObject>) -> Result<CallToolResult, String> {
        debug!("CharacterGetWorkingInfo {:?}", arg);
        let res = Character::load(&self.name).await;
        if res.is_err() {
            return Err("不存在此角色数据".into());
        }
        let character = res.unwrap();
        return Ok(CallToolResult {
            content: vec![Annotated::new(
                RawContent::Text(RawTextContent {
                    text: serde_json::to_string(&character).unwrap(),
                }),
                None,
            )],
            is_error: None,
        });
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
        "获取工作信息".into()
    }

    fn get_name(&self) -> String {
        "get_working_info".into()
    }
}

/// 角色自身设置工作任务
pub struct CharacterSetSelfTasks {
    pub name: String,
}

#[async_trait]
impl InternalFunctionCall for CharacterSetSelfTasks {
    async fn call(&self, arg: Option<JsonObject>) -> Result<CallToolResult, String> {
        debug!("CharacterSetTasks {:?}", arg);
        if arg.is_none() {
            return Err("缺少参数".into());
        }
        let arg = arg.unwrap();
        if !arg.contains_key("tasks") {
            return Err("缺少参数 tasks".into());
        }
        if !arg.contains_key("target") {
            return Err("缺少参数 target".into());
        }
        let character = Character::load(&self.name).await;
        if character.is_err() {
            return Err(format!("设置任务：不存在这个人物 {}", self.name).into());
        }
        let tmp = arg.get("tasks").unwrap();
        let tasks = serde_json::from_value(tmp.clone());
        if tasks.is_err() {
            return Err("参数 tasks 转换失败，请检查格式".into());
        }
        let target = arg.get("target").unwrap().as_str();
        if target.is_none() {
            return Err("参数 target 格式不正确".into());
        }
        let target = target.unwrap();
        let mut character = character.unwrap();
        character.working = CharacterWorkingInfo {
            target: target.to_string(),
            tasks: tasks.unwrap(),
            is_working: false,
            current_task_index: 0,
        };
        if let Err(e) = write_character(character).await {
            return Err(e);
        }
        Ok(CallToolResult {
            content: vec![Annotated::new(
                RawContent::Text(RawTextContent {
                    text: serde_json::to_string("成功").unwrap(),
                }),
                None,
            )],
            is_error: None,
        })
    }

    fn get_input_schema(&self) -> Arc<JsonObject> {
        Arc::new(
            serde_json::from_str(
                r#"
        {
            "properties":{
                "target": {
                    "description":"工作目标目标",
                    "type": "string",
                },
                "tasks":{
                    "description":"工作任务列表",
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "target": {
                                "type": "string",
                                "description": "任务目标"
                            }
                        }
                    }
                }
            }
        }"#,
            )
            .unwrap(),
        )
    }

    fn get_desc(&self) -> String {
        "设置自己的任务列表".into()
    }

    fn get_name(&self) -> String {
        "set_tasks".into()
    }
}

pub struct CharacterReadFile {
    pub name: String,
}

#[async_trait]
impl InternalFunctionCall for CharacterReadFile {
    async fn call(&self, arg: Option<JsonObject>) -> Result<CallToolResult, String> {
        debug!("CharacterSetTasks {:?}", arg);
        if arg.is_none() {
            return Err("缺少参数".into());
        }
        let arg = arg.unwrap();
        if !arg.contains_key("path") {
            return Err("缺少参数 path".into());
        }
        let path = arg.get("path").unwrap().as_str();
        if path.is_none() {
            return Err("参数 path 格式不正确".into());
        }
        let path = path.unwrap();
        if path.contains("..") {
            return Err("不得包含上级路径 \"..\"".into());
        }

        let res = read_role_directory(&self.name, path);
        if res.is_err() {
            return Err(res.unwrap_err().to_string());
        }
        Ok(CallToolResult {
            content: vec![Annotated::new(
                RawContent::Text(RawTextContent { text: res.unwrap() }),
                None,
            )],
            is_error: None,
        })
    }

    fn get_input_schema(&self) -> Arc<JsonObject> {
        Arc::new(
            serde_json::from_str(
                r#"
        {
            "properties":{
                "path": {
                    "description":"文件相对路径",
                    "type": "string",
                }
            }
        }"#,
            )
            .unwrap(),
        )
    }

    fn get_desc(&self) -> String {
        "在工作空间中读取指定相对路径的文件内容".into()
    }

    fn get_name(&self) -> String {
        "read_file".into()
    }
}

pub struct CharacterWriteFile {
    pub name: String,
}

#[async_trait]
impl InternalFunctionCall for CharacterWriteFile {
    async fn call(&self, arg: Option<JsonObject>) -> Result<CallToolResult, String> {
        debug!("CharacterSetTasks {:?}", arg);
        if arg.is_none() {
            return Err("缺少参数".into());
        }
        let arg = arg.unwrap();
        if !arg.contains_key("path") {
            return Err("缺少参数 path".into());
        }
        if !arg.contains_key("content") {
            return Err("缺少参数 content".into());
        }
        let path = arg.get("path").unwrap().as_str();
        if path.is_none() {
            return Err("参数 path 格式不正确".into());
        }
        let path = path.unwrap();
        if path.contains("..") {
            return Err("不得包含上级路径 \"..\"".into());
        }
        let content = arg.get("content").unwrap().as_str();
        if content.is_none() {
            return Err("参数 content 格式不正确".into());
        }
        let content = content.unwrap();

        let res = store_to_role_directory(&self.name, path, content);
        if res.is_err() {
            return Err(res.unwrap_err().to_string());
        }
        Ok(CallToolResult {
            content: vec![Annotated::new(
                RawContent::Text(RawTextContent {
                    text: "成功".into(),
                }),
                None,
            )],
            is_error: None,
        })
    }

    fn get_input_schema(&self) -> Arc<JsonObject> {
        Arc::new(
            serde_json::from_str(
                r#"
        {
            "properties":{
                "path": {
                    "description":"文件相对路径",
                    "type": "string",
                },
                "content": {
                    "description":"文件内容",
                    "type": "string",
                }
            }
        }"#,
            )
            .unwrap(),
        )
    }

    fn get_desc(&self) -> String {
        "在指定相对路径写入文件内容".into()
    }

    fn get_name(&self) -> String {
        "write_file".into()
    }
}
