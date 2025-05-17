use std::fmt::Debug;
use super::life::Life;
use super::rolecard::RoleCard;

#[derive(Debug, Clone)]
pub struct ThinkConfig {
    pub pulse_interval: u64,
}

pub struct Think {
    pub config: ThinkConfig,
    pub life: Life,
    pub rolecard: RoleCard,
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
                // role: String::new(),
            },
            life: Life::new(),
            rolecard: RoleCard::new(),
        }
    }

    pub fn set_rolecard(&mut self, rolecard: RoleCard) {
        self.rolecard = rolecard;
    }

    pub fn get_think_string(&self)->String {
        let mut ret = self.rolecard.get_prompt();
        ret += &self.life.get_schedule().get_daily_schedules_string();
        ret += "现在你已经被唤醒";
        ret
    }
}