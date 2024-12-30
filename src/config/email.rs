use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EmailConfig {
    pub smtp: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub from: String,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            smtp: "smtp.exmail.qq.com".to_string(),
            port: 465,
            username: "gitdata-bot@gitdata.ai".to_string(),
            password: "******".to_string(),
            from: "gitdata-bot@gitdata.ai".to_string(),
        }
    }
}
