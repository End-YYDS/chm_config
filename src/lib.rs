use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub hostname: String,
    pub path: String,
    pub password: String,
    pub kind: String,
    pub tools: Vec<String>,
}

#[allow(dead_code)]
impl Config {
    fn new(
        hostname: String,
        path: String,
        password: String,
        kind: String,
        tools: Vec<String>,
    ) -> Self {
        Self {
            hostname,
            path,
            password,
            kind,
            tools,
        }
    }
}
