use serde::{Deserialize, Serialize};
use tracing::log;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PgConfig {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub schema: String,
    pub pool_size: u32,
    pub max_conn_lifetime: u64,
    pub max_conn_lifetime_ms: u64,
    pub idle_timeout: u64,
    pub connect_timeout: u64,
    pub max_connections: u32,
    pub min_connections: u32,
    pub log_level: String,
}

impl Default for PgConfig {
    fn default() -> Self {
        PgConfig {
            host: "localhost".to_string(),
            port: "5432".to_string(),
            user: "postgres".to_string(),
            password: "postgres".to_string(),
            dbname: "postgres".to_string(),
            schema: "public".to_string(),
            pool_size: 10,
            max_conn_lifetime: 60,
            max_conn_lifetime_ms: 60000,
            idle_timeout: 60,
            connect_timeout: 10,
            max_connections: 10,
            min_connections: 1,
            log_level: "info".to_string(),
        }
    }
}

impl PgConfig {
    pub fn format(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.dbname
        )
    }
    pub fn level(&self) -> log::LevelFilter {
        match self.log_level.to_lowercase().as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Info,
        }
    }
}
