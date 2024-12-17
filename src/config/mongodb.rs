use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct MongoDBConfig{
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
        MongoDBConfig{
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
