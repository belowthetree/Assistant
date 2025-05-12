use serde::{Deserialize, Serialize};
use reqwest::Client;
use rmcp::model::Tool;
use async_trait::async_trait;

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
}

impl ModelResponse {
    pub fn tostring(&self)->String {
        serde_json::to_string(&serde_json::json!(self.clone())).unwrap()
    }
}

#[async_trait]
pub trait ModelInteract {
    fn get_model_name(&self) -> String;
    fn set_api_key(&mut self, key: String);
    async fn check_api_key_valid(&self) -> Result<bool, String>;
    async fn generate(&self, param: ModelInputParam) -> Result<ModelResponse, String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EModelType {
    Deepseek,
    Ollama,
    OpenAI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelData {
    pub model_type: EModelType,
    pub api_key: String,
    pub url: String,
    pub model_name: String,
    pub temperature: f64,
}

impl ModelData {
    pub fn new(url: String, model_name: String, api_key: String, model_type: EModelType) -> Self {
        Self {
            model_type,
            api_key,
            url,
            model_name,
            temperature: 0.6,
        }
    }

    fn get_model_name(&self) -> String {
        self.model_name.clone()
    }
}
