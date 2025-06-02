use rmcp::model::Tool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMessage {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tool_call_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInputParam {
    pub content: Option<String>,
    pub system: Option<String>,
    pub temperature: Option<f64>,
    pub tools: Option<Vec<Tool>>,
    pub messages: Option<Vec<ModelMessage>>,
}

fn _default_tool_call_function() -> ToolCallFunction {
    ToolCallFunction::new()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    #[serde(default)]
    pub index: usize,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default = "_default_tool_call_function")]
    pub function: ToolCallFunction,
}

impl ToolCall {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            index: 0,
            id: String::new(),
            r#type: String::new(),
            function: ToolCallFunction::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallFunction {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub arguments: String,
}

impl ToolCallFunction {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            arguments: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    pub role: String,
    pub content: String,
    pub reasoning_content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EModelType {
    Deepseek,
    Ollama,
    OpenAI,
}

impl Default for EModelType {
    fn default() -> Self {
        EModelType::Deepseek
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelData {
    pub model_type: EModelType,
    pub api_key: String,
    pub url: String,
    pub model_name: String,
    pub temperature: String,
    #[serde(default)]
    pub stream: bool,
}

impl ModelData {
    pub fn new(url: String, model_name: String, api_key: String, model_type: EModelType) -> Self {
        Self {
            model_type,
            api_key,
            url,
            model_name,
            temperature: "0.6".into(),
            stream: true,
        }
    }
}
