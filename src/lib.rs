use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    hostname: String,
    path: String,
    password: String,
    kind: String,
    tools: Vec<String>,
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
