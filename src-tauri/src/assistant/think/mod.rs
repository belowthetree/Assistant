use std::fmt::Debug;
use super::life::Life;
use super::rolecard::RoleCard;

#[derive(Debug, Clone)]
pub struct ThinkConfig {
    pub pulse_interval: u64,
    pub role: RoleCard,
}

pub struct Think {
    pub config: ThinkConfig,
    pub life: Life,
}

impl Debug for Think {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Think").field("config", &self.config).finish()
    }
}

impl Think {
    pub fn new()->Self {
        Self {
            config: ThinkConfig {
                pulse_interval: 240,
                role: RoleCard::new(),
            },
            life: Life::new(),
        }
    }

    pub fn get_think_string(&self)->String {
        let mut ret = self.config.role.get_prompt();
        ret += &self.life.get_schedule().get_daily_schedules_string();
        ret += "现在你已经被唤醒";
        ret
    }
}