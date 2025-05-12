use schedule::Schedule;

mod schedule;

#[derive(Debug, Clone)]
pub struct Life {
    pub sche: schedule::Schedule,
}

impl Life {
    pub fn new()->Self {
        Self {
            sche: Schedule::new()
        }
    }
}