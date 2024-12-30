use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MongoDBConfig {
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub database: String,
    pub auth_db: String,
    pub pool_size: i32,
}

impl Default for MongoDBConfig {
    fn default() -> Self {
        MongoDBConfig {
            host: "localhost".to_string(),
            port: 27017,
            username: "".to_string(),
            password: "".to_string(),
            database: "".to_string(),
            auth_db: "".to_string(),
            pool_size: 10,
        }
    }
}

impl MongoDBConfig {
    pub fn format(&self) -> String {
        format!("mongodb://{}:{}@{}:{}/?directConnection=true&serverSelectionTimeoutMS=2000&maxPoolSize={}&minPoolSize={}", 
                self.username,
                self.password,
                self.host,
                self.port,
                self.pool_size,
                self.pool_size / 10
        )
    }
}
