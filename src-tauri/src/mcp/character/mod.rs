use serde::{Deserialize, Serialize};

use crate::assistant::RoleCard;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterTask {
    pub target: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterWorkingInfo {
    pub target: String,
    pub tasks: Vec<CharacterTask>,
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
                current_task_index: 0,
            },
            rolecard,
        }
    }
}
