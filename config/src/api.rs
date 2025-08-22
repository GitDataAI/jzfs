use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct AppApiConfig {
    #[serde(rename = "host")]
    pub host: String,
    #[serde(rename = "port")]
    pub port: u16,
    #[serde(rename = "session")]
    pub session: String,
    #[serde(rename = "session_max_age")]
    pub max_age: i64,
    #[serde(rename = "session_secret")]
    pub secret: String,
    #[serde(rename = "workers")]
    pub workers: usize,
}

impl AppApiConfig {
    pub fn new() -> Self {
        AppApiConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            session: "session".to_string(),
            max_age: 86400,
            secret: "secret".to_string(),
            workers: 16,
        }
    }
}
impl Default for AppApiConfig {
    fn default() -> Self {
        AppApiConfig::new()
    }
}
