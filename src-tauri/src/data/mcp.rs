use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::mcp::MCPServerConfig;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerData {
    pub servers: HashMap<String, MCPServerConfig>,
}

impl ServerData {
    pub fn new()->Self {
        Self {
            servers: HashMap::new(),
        }
    }
}