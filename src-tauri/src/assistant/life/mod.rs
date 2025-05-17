use schedule::Schedule;

mod schedule;

#[derive(Debug, Clone)]
pub struct Life {
    sche: schedule::Schedule,
}

impl Life {
    pub fn new()->Self {
        Self {
            sche: Schedule::new()
        }
    }

    pub fn get_schedule(&self)->Schedule {
        self.sche.clone()
    }
}