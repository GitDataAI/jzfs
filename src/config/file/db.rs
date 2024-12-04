use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct DBConfig{
    pub driver: String,
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl Default for DBConfig {
    fn default() -> Self {
        Self{
            driver: "postgres".to_string(),
            host: "localhost".to_string(),
            port: "6349".to_string(),
            user: "postgres".to_string(),
            password: "postgres".to_string(),
            database: "postgres".to_string(),
        }
    }
}

impl DBConfig {
    pub fn format(&self) -> String{
        format!("{}://{}:{}@{}:{}/{}",
            self.driver,
            self.user,
            self.password,
            self.host,
            self.port,
            self.database
        )
    }
}