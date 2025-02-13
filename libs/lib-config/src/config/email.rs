use std::io;

use serde::Deserialize;
use serde::Serialize;

use crate::config::AppConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailConfig {
    pub smtp : String,
    pub port : i64,
    pub username : String,
    pub password : String,
    pub from : String,
}

impl AppConfig {
    pub async fn get_email_config(&self, server : &str) -> io::Result<EmailConfig> {
        let data_id = format!("email.{}", server);
        if let Ok(data) = self.client.get_config(data_id, "email".to_string()).await {
            if let Ok(data) = serde_json::from_str::<EmailConfig>(&data.content()) {
                Ok(data)
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "parse config error"))
            }
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "get config error"))
        }
    }
}
