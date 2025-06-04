use ::serde::{Deserialize, Serialize};

use crate::model::{ModelMessage, ModelResponse, ToolCall};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ERole {
    Assistant,
    User,
    System,
    Tool,
}

impl Default for ERole {
    fn default() -> Self {
        Self::Assistant
    }
}

impl ERole {
    pub fn to_string(&self) -> String {
        match self.clone() {
            ERole::Assistant => "assistant".into(),
            ERole::User => "user".into(),
            ERole::System => "system".into(),
            ERole::Tool => "tool".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(unused)]
pub struct TalkContent {
    pub role: ERole,
    pub content: String,
    pub reasoning_content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TalkContext {
    content: Vec<TalkContent>,
    max_conent: usize,
}

impl TalkContext {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            max_conent: 10,
        }
    }

    pub fn get_messages(&self, system: Option<&TalkContent>) -> Vec<ModelMessage> {
        let mut res: Vec<ModelMessage> = Vec::new();
        if let Some(sys) = system {
            res.push(ModelMessage {
                role: sys.role.to_string(),
                content: sys.content.clone(),
                name: "".into(),
                tool_call_id: "".into(),
                tool_calls: None,
            });
        }
        for ctx in self.content.iter() {
            res.push(ModelMessage {
                role: ctx.role.to_string(),
                content: ctx.content.clone(),
                name: ctx.clone().name.unwrap_or_default(),
                tool_call_id: ctx.clone().tool_call_id.unwrap_or_default(),
                tool_calls: ctx.tool_calls.clone(),
            });
        }
        res
    }

    pub fn add_assistant(&mut self, response: &ModelResponse) {
        self.content.push(TalkContent {
            role: ERole::Assistant,
            content: response.content.clone(),
            reasoning_content: response.reasoning_content.clone(),
            tool_calls: response.tool_calls.clone(),
            name: None,
            tool_call_id: None,
        });
        if self.content.len() > self.max_conent {
            self.content.remove(0);
        }
    }

    pub fn add_content(&mut self, content: &TalkContent) {
        self.content.push(content.clone());
        if self.content.len() > self.max_conent {
            self.content.remove(0);
        }
    }

    pub fn add_user(&mut self, content: String) {
        self.content.push(TalkContent {
            role: ERole::User,
            content,
            reasoning_content: None,
            tool_calls: None,
            name: None,
            tool_call_id: None,
        });
        if self.content.len() > self.max_conent {
            self.content.remove(0);
        }
    }

    pub fn add_system(&mut self, content: String) {
        self.content.push(TalkContent {
            role: ERole::System,
            content,
            reasoning_content: None,
            tool_calls: None,
            name: None,
            tool_call_id: None,
        });
        if self.content.len() > self.max_conent {
            self.content.remove(0);
        }
    }

    pub fn clear(&mut self) {
        self.content.clear();
    }
}
