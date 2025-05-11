mod modelbase;
mod deepseek;
mod ollama;

use log::debug;
pub use modelbase::*;
pub use deepseek::*;
pub use ollama::*;

#[tauri::command]
pub async fn test() {
    let  url = format!("https://api.deepseek.com/v1");
    let model_name = format!("deepseek-chat");
    let api_key = format!("sk-25c8486584eb474bb3d175a61d503201");
    let data = ModelData::new(url, model_name, api_key, EModelType::Deepseek);
    let res = deepseek::Deepseek::generate(&data, ModelInputParam {
        content: Some("你好".to_string()), system: None, temperature: None, tools: None, messages: None
    }).await;
    debug!("{:?}", res);
}