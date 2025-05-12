use log::{debug, warn};
use reqwest::{header, Client};

use super::{ModelData, ModelInputParam, ModelResponse};


pub trait Ollama {
    async fn get_models(&self) -> Result<Vec<String>, String>;
    async fn generate(&self, param: ModelInputParam) -> Result<ModelResponse, String>;
}

impl Ollama for ModelData {
    async fn get_models(&self) -> Result<Vec<String>, String> {
        let client = Client::new();
        let response = client
            .get(format!("{}/api/tags", self.url))
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &self.api_key)
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

    async fn generate(&self, param: ModelInputParam) -> Result<ModelResponse, String> {
        debug!("ollama 接口开始发送，模型：{}，参数：{:?}", self.model_name, param);
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
                warn!("报错：{:?}", text);
                Err(text)
            }
        } else {
            warn!("报错：{:?}", response);
            Err(response.text().await.map_err(|e| e.to_string())?)
        }
    }
}