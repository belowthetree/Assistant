use serde::{Deserialize, Serialize};
use rmcp::model::Tool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInputParam {
    pub content: Option<String>,
    pub system: Option<String>,
    pub temperature: Option<f64>,
    pub tools: Option<Vec<Tool>>,
    pub messages: Option<Vec<ModelMessage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: ToolCallFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    pub role: String,
    pub content: String,
    pub reasoning_content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub finish_reason: Option<String>,
}

impl ModelResponse {
    pub fn tostring(&self)->String {
        serde_json::to_string(&serde_json::json!(self.clone())).unwrap()
    }
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

    fn get_model_name(&self) -> String {
        self.model_name.clone()
    }
}
