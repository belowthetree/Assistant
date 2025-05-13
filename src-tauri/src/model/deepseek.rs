use super::{ModelData, ModelInputParam, ModelResponse};
use futures_util::{StreamExt};
use log::debug;
use reqwest::{header, Client};
use serde_json::Value;

// 定义回调函数类型
pub type SseCallback = Box<dyn FnMut(String) + Send>;

pub trait Deepseek {
    async fn generate(&self, param: ModelInputParam, stream_callback: Option<SseCallback>) -> Result<ModelResponse, String>;
    async fn get_models(&self) -> Result<Vec<String>, String>;
    fn get_api_key(&self)->String;
}

impl Deepseek for ModelData {
    // stream_callback 用于流式回调,可选
    async fn generate(&self, param: ModelInputParam, mut stream_callback: Option<SseCallback>) -> Result<ModelResponse, String> {
        debug!("{:?}", param);
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
                "stream": self.stream,
                "temperature": param.temperature.unwrap_or(self.temperature.parse().unwrap())
            })).unwrap())
            .send()
            .await
            .map_err(|e| e.to_string()
        )?;

        let succ = response.status().is_success();
        debug!("{:?}", response);
        let mut text = "".into();
        if succ {
            // 区分流式和非流式
            if self.stream {
                let mut stream = response.bytes_stream();
                let mut content = String::new();
                
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk.map_err(|e| e.to_string())?;
                    let chunk_str = String::from_utf8_lossy(&chunk);
                    
                    // 解析SSE格式的数据
                    for line in chunk_str.lines() {
                        if line.starts_with("data:") {
                            let data = line.trim_start_matches("data:").trim();
                            if data == "[DONE]" {
                                break;
                            }
                            
                            if let Ok(json) = serde_json::from_str::<Value>(data) {
                                if let Some(choices) = json.get("choices") {
                                    for choice in choices.as_array().unwrap_or(&vec![]) {
                                        if let Some(finish_reason) = choice.get("finish_reason") {
                                            if finish_reason.as_str() == Some("stop") {
                                                break;
                                            }
                                        }
                                        
                                        if let Some(index) = choice.get("index") {
                                            // 处理索引信息
                                        }
                                        
                                        if let Some(message) = choice.get("message") {
                                            if let Some(ctx) = message.get("content") {
                                                if let Some(text_str) = ctx.as_str() {
                                                    content.push_str(text_str);
                                                }
                                            }
                                            if let Some(reasoning_content) = message.get("reasoning_content") {
                                                // 处理推理内容
                                            }
                                            if let Some(tool_calls) = message.get("tool_calls") {
                                                // 处理工具调用
                                            }
                                        } else if let Some(delta) = choice.get("delta") {
                                            // 兼容旧版 delta 格式
                                            if let Some(text) = delta.get("content") {
                                                if let Some(text_str) = text.as_str() {
                                                    content.push_str(text_str);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                return Ok(ModelResponse {
                    role: "assistant".to_string(),
                    content,
                    reasoning_content: None,
                    tool_calls: None,
                    finish_reason: None,
                });
            }
            else {
                text = response.text().await.unwrap();
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
                            
                            let reasoning_content = message.get("reasoning_content")
                                .and_then(Value::as_str)
                                .map(|s| s.to_string());
                            
                            let finish_reason = first_choice.get("finish_reason")
                                .and_then(Value::as_str)
                                .map(|s| s.to_string());
                            
                            let index = first_choice.get("index")
                                .and_then(Value::as_u64);
                            
                            return Ok(ModelResponse {
                                role: "assistant".to_string(),
                                content,
                                reasoning_content,
                                tool_calls,
                                finish_reason,
                            });
                        }
                    }
                }
            }
        }

        debug!("API 请求失败");
        Err(text)
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
