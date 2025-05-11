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
    pub messages: Vec<ModelMessage>,
    pub url: String,
    pub model_name: String,
    pub temperature: f64,
    // TODO: Add role_card implementation
}

impl ModelData {
    pub fn new(url: String, model_name: String, api_key: String, model_type: EModelType) -> Self {
        Self {
            model_type,
            api_key,
            messages: Vec::new(),
            url,
            model_name,
            temperature: 0.6,
        }
    }
}

#[async_trait]
impl ModelInteract for ModelData {
    fn get_model_name(&self) -> String {
        self.model_name.clone()
    }

    fn set_api_key(&mut self, key: String) {
        self.api_key = key;
    }

    async fn check_api_key_valid(&self) -> Result<bool, String> {
        Ok(true)
    }

    async fn generate(&self, param: ModelInputParam) -> Result<ModelResponse, String> {
        if !self.check_api_key_valid().await? {
            return Err(format!("API Key 未设置 for model {}", self.get_model_name()));
        }

        let client = Client::new();
        let response = client
            .post(self.url.clone())
            .header("Content-Type", "application/json")
            .header("Authorization", &self.api_key)
            .body(serde_json::to_string(&serde_json::json!({
                "model": self.model_name,
                "system": param.system.unwrap_or_default(),
                "prompt": param.content.unwrap_or_default(),
                "stream": false,
                "options": {
                    "temperature": param.temperature.unwrap_or(self.temperature)
                },
                "tools": param.tools,
            })).unwrap())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            let text = response.text().await.map_err(|e| e.to_string())?;
            let js: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
            
            if let Some(message) = js.get("message") {
                let res: ModelResponse = serde_json::from_value(message.clone())
                    .map_err(|e| e.to_string())?;
                Ok(res)
            } else if let Some(response) = js.get("response") {
                let res: ModelResponse = serde_json::from_value(response.clone())
                    .map_err(|e| e.to_string())?;
                Ok(res)
            } else {
                Err(text)
            }
        } else {
            Err(response.text().await.map_err(|e| e.to_string())?)
        }
    }
}
