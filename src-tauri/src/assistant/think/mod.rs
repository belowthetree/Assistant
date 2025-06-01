use log::debug;

use super::life::Life;
use super::rolecard::RoleCard;
use std::fmt::Debug;

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
        f.debug_struct("Think")
            .field("config", &self.config)
            .finish()
    }
}

impl Think {
    pub fn new() -> Self {
        Self {
            config: ThinkConfig {
                pulse_interval: 3600,
                // role: String::new(),
            },
            life: Life::new(),
            rolecard: RoleCard::new(),
        }
    }

    pub fn set_rolecard(&mut self, rolecard: RoleCard) {
        debug!("设置角色卡 {:?}", rolecard);
        self.rolecard = rolecard;
    }

    pub async fn get_think_string(&self) -> String {
        let mut ret = self.rolecard.get_prompt();
        ret += &self.life.get_schedule_string().await;
        ret += "现在你已经被唤醒";
        ret
    }

    pub fn get_conversation_string(&self) -> String {
        self.rolecard.get_prompt()
    }
}
