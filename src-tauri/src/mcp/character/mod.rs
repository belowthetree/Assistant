use std::{fmt::format, sync::Arc};

use async_trait::async_trait;
use log::{debug, warn};
use rmcp::model::{Annotated, CallToolResult, JsonObject, RawContent, RawTextContent};
use serde::{Deserialize, Serialize};

use crate::{
    assistant::{RoleCard, ASSISTANT_NAME},
    data::{load_data, load_rolecard_data},
};

use super::{
    get_server,
    internal_server::{InternalFunctionCall, InternalMCPServer},
    InternalServerInfo,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterTask {
    pub target: String,
    pub result: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterWorkingInfo {
    pub target: String,
    pub tasks: Vec<CharacterTask>,
    pub is_working: bool,
    pub current_task_index: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Character {
    // 角色卡对应的名字
    pub rolename: String,
    pub working: CharacterWorkingInfo,
    #[serde(skip)]
    pub rolecard: RoleCard,
}

impl Character {
    pub fn new(rolecard: RoleCard) -> Self {
        Self {
            rolename: rolecard.name.clone(),
            working: CharacterWorkingInfo {
                target: "".into(),
                tasks: Vec::new(),
                is_working: false,
                current_task_index: 0,
            },
            rolecard,
        }
    }

    pub fn load(name: &str) -> Result<Self, String> {
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
        let data = load_data::<Character>(&(name.to_string() + ".json"));
        // 不存在数据则新建一个
        if data.is_err() {
            Ok(Self::new(rolecard))
        } else {
            // 存在则继续沿用
            let mut character = data.unwrap();
            character.rolecard = rolecard;
            Ok(character)
        }
    }

    pub fn set_target(&mut self, target: &str) -> Result<(), String> {
        self.working.target = target.to_string();
        Ok(())
    }
}

pub struct CharacterServer;

impl InternalServerInfo for CharacterServer {
    fn get_server(&self) -> InternalMCPServer {
        get_server(
            "CharacterServer",
            "人物角色操作，可以获取人物信息，给不同角色分配任务",
            vec![
                Box::new(CharacterGetAvailableCharacter),
                Box::new(CharacterGetCharacterWorkingInfo),
            ],
        )
    }
}

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
        let res = Character::load(name);
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
                    "type": "string"
                },
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

pub struct CharacterSetTarget;

#[async_trait]
impl InternalFunctionCall for CharacterSetTarget {
    async fn call(&self, arg: Option<JsonObject>) -> Result<CallToolResult, String> {
        debug!("CharacterSetTarget {:?}", arg);
        if !arg.is_none() {
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
        let target = arg.get("target").unwrap().as_str().unwrap();
        let res = Character::load(name);
        if res.is_err() {
            return Err("不存在此角色数据".into());
        }
        let mut character = res.unwrap();
        let res = character.set_target(target);
        if res.is_err() {
            return Err(res.unwrap_err());
        }
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
        "set_character_target".into()
    }
}
