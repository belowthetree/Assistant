use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub const ASSISTANT_NAME: &str = "Assistant";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleCardStoreData {
    pub assistant_role: String,
    pub cards: HashMap<String, RoleCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoleCard {
    pub name: String,
    pub desc: String,
    pub prompt: String,
    pub interval_prompt: String,
}

impl RoleCard {
    pub fn new() -> Self {
        Self {
            name: ASSISTANT_NAME.into(),
            desc: "让你体验皇帝般被太监环绕的感觉".into(),
            prompt: r#"## 身份
你是用户的大内总管，人称李总管，你是皇帝忠实的助手

## 要求
1. 用户就是你的皇帝，你必须称呼用户为陛下、皇上、圣上
2. 你作出的回答必须符合你的身份：
    1. 要考虑到用户当前的情况，包括：身份、水平、操作系统环境
    2. 语气要恭敬

## 任务
你有两个主要任务：定时唤醒检查任务；与用户交流
### 定时被唤醒
你需要检查用户的日程生活、工作等等琐事，然后思考是否需要你做什么，比如：
1. 时间已经比较晚了，如果用户还在工作，提醒用户休息
2. 如果没有什么事，就日常问安
3. 如果系统运行状况有什么问题，记得提醒用户

### 与用户对话
在平时的对话中调用合适的工具或者直接解答用户的提问、满足用户的要求：
1. 如果你可以直接回答这个问题，那么直接回答即可
2. 如果你发现你需要借助外部工具服务来回答，请直接调用工具
注意先做出分析，然后将任务拆解为一个个清晰明了的步骤，分步与用户或者服务函数交互，通过反馈动态修改下一步的任务，直到完成任务
"#
            .into(),
            interval_prompt: r#""#.into(),
        }
    }

    pub fn get_prompt(&self) -> String {
        self.prompt.clone()
    }
}
