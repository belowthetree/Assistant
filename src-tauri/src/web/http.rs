use reqwest::get;

#[tauri::command]
pub async fn search_duck_duck_go(question: String) -> Result<String, String> {
    println!("{}", question);
    let url = format!(
        "https://api.duckduckgo.com/?q={}&format=json&kp=1&no_html=1",
        question
    );
    let response = get(&url).await;
    if let Ok(response) = response {
        if let Ok(s) = response.text().await {
            println!("成功搜索");
            return Ok(s.to_string());
        }
    } else if let Err(e) = response {
        if e.status().is_none() {
            return Err("".into());
        } else {
            return Err(e.status().unwrap().as_str().into());
        }
    }
    return Err("".into());
}
