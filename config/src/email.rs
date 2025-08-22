use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct AppEmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_sender: String,
}

impl Default for AppEmailConfig {
    fn default() -> Self {
        Self {
            smtp_host: "...".to_string(),
            smtp_port: 587,
            smtp_username: "...".to_string(),
            smtp_password: "...".to_string(),
            smtp_sender: "...".to_string(),
        }
    }
}
