use std::fmt::format;

use super::{ModelData, ModelInputParam, ModelResponse};
use log::debug;
use reqwest::{header, Client};
use serde_json::Value;

pub trait Deepseek {
    async fn generate(&self, param: ModelInputParam) -> Result<ModelResponse, String>;
    async fn get_models(&self) -> Result<Vec<String>, String>;
    fn get_api_key(&self)->String;
}

impl Deepseek for ModelData {
    async fn generate(&self, param: ModelInputParam) -> Result<ModelResponse, String> {
        let mut messages = param.messages.unwrap_or_default();

        if messages.is_empty() {
            messages.push(super::ModelMessage {
                role: "system".to_string(),
                content: param.system.unwrap_or_default(),
            });
            messages.push(super::ModelMessage {
                role: "user".to_string(),
                content: param.content.unwrap_or_default(),
            });
        }

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/chat/completions", self.url))
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", self.get_api_key())
            .body(serde_json::to_string(&serde_json::json!({
                "model": self.model_name,
                "messages": messages,
                "stream": false,
                "temperature": param.temperature.unwrap_or(self.temperature)
            })).unwrap())
            .send()
            .await
            .map_err(|e| e.to_string()
        )?;

        let succ = response.status().is_success();
        let text = response.text().await.unwrap_or_default();
        if succ {
            let json: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

            if let Some(choices) = json.get("choices") {
                if let Some(first_choice) = choices.get(0) {
                    if let Some(message) = first_choice.get("message") {
                        let content = message.get("content")
                            .and_then(Value::as_str)
                            .unwrap_or_default()
                            .to_string();
                        
                        let tool_calls = message.get("tool_calls")
                            .and_then(|v| serde_json::from_value(v.clone()).ok());
                        
                        return Ok(ModelResponse {
                            role: "assistant".to_string(),
                            content,
                            reasoning_content: None,
                            tool_calls,
                        });
                    }
                }
            }
        }

        debug!("{:?}", text);
        let error_text = text;
        Err(error_text)
    }
    
    async fn get_models(&self) -> Result<Vec<String>, String> {
        let client = Client::new();
        let response = client
            .get(format!("{}/models", self.url))
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", self.get_api_key())
            .body(serde_json::to_string(&serde_json::json!({
            })).unwrap())
            .send()
            .await
            .map_err(|e| e.to_string()
        )?;

        let succ = response.status().is_success();
        let text = response.text().await;
        if succ {
            let text = text.map_err(|e| e.to_string())?;
            let js: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
            
            if let Some(message) = js.get("data") {
                let mut res: Vec<String> = Vec::new();
                for val in message.as_array().unwrap().iter() {
                    if let Some(id) = val.get("id") {
                        res.push(serde_json::from_value(id.clone()).unwrap());
                    }
                }
                return Ok(res);
            }
            else {
                debug!("{:?}", text);
                return Err(text);
            }
        }
        else {
            debug!("{:?}", text);
            return Err(text.map_err(|e| e.to_string())?);
        }
    }
    
    fn get_api_key(&self)->String {
        format!("Bearer {}", self.api_key)
    }
}
