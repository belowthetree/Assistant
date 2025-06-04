#[derive(Debug, Clone)]
pub struct StepTalkInfo {
    is_stepping: bool,
    max_step: u64,
    current_step: u64,
    steps: Vec<OneStepInfo>,
}

#[derive(Debug, Clone)]
pub struct OneStepInfo {
    pub target: String,
    pub result: String,
}

impl StepTalkInfo {
    pub fn new() -> Self {
        Self {
            is_stepping: false,
            max_step: 11,
            current_step: 0,
            steps: Vec::new(),
        }
    }
    pub fn add_step(&mut self, step: OneStepInfo) -> Result<(), ()> {
        if self.current_step >= self.max_step || !self.is_stepping {
            self.current_step = 0;
            Err(())
        } else {
            self.steps.push(step);
            Ok(())
        }
    }
}
