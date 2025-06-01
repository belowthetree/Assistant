use crate::mcp::{ScheItem, SCHEDULE};

#[derive(Debug, Clone)]
pub struct Life {}

impl Life {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_schedule_string(&self) -> String {
        let mut sche = SCHEDULE.lock().await;
        let prefix: String = r#"## 日程安排（可能为空）"#.into();
        prefix + &serde_json::to_string(&sche.get_sches()).unwrap() + "\n"
    }
}
