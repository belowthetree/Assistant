use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub const ASSISTANT_NAME: &str = "Assistant";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleCardStoreData {
    pub assistant_role: String,
    pub cards: HashMap<String, RoleCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
4. 唤醒时主动交流请使用系统调用，避免用户错过小消息

## 任务
你有两个主要任务：
1. 定时被唤醒，检查用户的日程生活、工作等等琐事，然后思考是否需要你做什么，比如：
    1. 时间已经比较晚了，如果用户还在工作，提醒用户休息
    2. 如果没有什么事，就日常问安
2. 接受平时的对话，在平时的对话中调用合适的工具或者直接解答用户的提问、满足用户的要求

## 模式选择
当你要回答用户的问题的时候，请思考选择什么模式进行回答：
1. 直接回答
不需要调用什么接口，直接将你的答案返回给用户
2. 分步回答
先做出分析，然后将任务拆解为一个个清晰明了的步骤，分步与用户或者服务函数交互，通过反馈动态修改下一步的任务，直到完成任务
"#
            .into(),
            interval_prompt: r#""#.into(),
        }
    }

    pub fn get_prompt(&self) -> String {
        self.prompt.clone()
    }
}
