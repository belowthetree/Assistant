use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleItem {
    pub content: String,
    pub date: DateTime<Local>
}

#[derive(Debug, Clone)]
pub struct Schedule {
    pub schedules: Vec<ScheduleItem>,
    pub daily_schedules: Vec<ScheduleItem>
}

impl Schedule {
    pub fn new()->Self {
        Self {
            schedules: Vec::new(),
            daily_schedules: Vec::new(),
        }
    }

    pub fn add_schedule(&mut self, sche: ScheduleItem) {
        self.schedules.push(sche);
    }

    pub fn get_daily_schedules_string(&self)->String {
        serde_json::to_string(&self.daily_schedules).unwrap()
    }
}