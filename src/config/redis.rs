use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct RedisConfig{
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub db: i64,
    pub pool_size: usize,
}

impl Default for RedisConfig{
    fn default() -> Self{
        RedisConfig{
            host: "127.0.0.1".to_string(),
            port: 6379,
            password: None,
            db: 0,
            pool_size: 10,
        }
    }
}

impl RedisConfig {
    pub fn format(&self) -> String{
        match &self.password{
            Some(password) => format!("redis://:{}@{}:{}/{}", password, self.host, self.port, self.db),
            None => format!("redis://{}:{}/{}", self.host, self.port, self.db)
        }
    }
}