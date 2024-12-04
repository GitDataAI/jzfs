use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct RedisConfig{
    pub host:String,
    pub port:String,
    pub password:String,
    pub db:String,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self{
            host: "127.0.0.1".to_string(),
            port: "6379".to_string(),
            password: "".to_string(),
            db: "0".to_string(),
        }
    }
}

impl RedisConfig {
    pub fn format(&self) -> String{
        if self.password.is_empty() {
            format!("redis://{}:{}", self.host, self.port)
        } else {
            format!("redis://:{}@{}:{}", self.password, self.host, self.port)
        }
    }
}