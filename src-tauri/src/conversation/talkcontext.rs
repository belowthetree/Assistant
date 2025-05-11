use serde::{Deserialize, Serialize};

use crate::model::ModelMessage;


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ERole {
    Assistant,
    User,
    System,
}

impl ERole {
    pub fn to_string(&self)->String {
        match self.clone() {
            ERole::Assistant => "assistant".into(),
            ERole::User => "user".into(),
            ERole::System => "system".into(),
        }
    }
}

pub struct ToolFunction {
    name: String,
    arguments: String
}

pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: ToolFunction
}

pub struct TalkContent {
    pub role: ERole,
    pub content: String,
    pub reasoning_content: String,
    pub tool_calls: Vec<ToolCall>,
}

pub struct TalkContext {
    content: Vec<TalkContent>
}

impl TalkContext {
    pub fn new()->Self {
        Self {
            content: Vec::new()
        }
    }

    pub fn get_messages(&self)->Vec<ModelMessage> {
        let mut res: Vec<ModelMessage> = Vec::new();
        for ctx in self.content.iter() {
            res.push(ModelMessage {
                role: ctx.role.to_string(),
                content: ctx.content.clone(),
            });
        }
        res
    }
}
