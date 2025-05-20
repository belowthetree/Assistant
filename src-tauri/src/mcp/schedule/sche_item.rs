use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EScheStatus {
    Unfinish,
    Finish,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheItem {
    pub title: String,
    pub content: String,
    pub status: EScheStatus,
    pub create_time: DateTime<Local>,
    pub finish_time: DateTime<Local>,
}