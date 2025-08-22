use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use tracing::log;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct AppDatabaseConfig {
    #[serde(rename = "type")]
    pub db_type: DatabaseType,
    #[serde(rename = "db_url")]
    pub db_url: String,
    #[serde(rename = "max_conn")]
    pub max_conn: u32,
    #[serde(rename = "lazy")]
    pub lazy: bool,
    #[serde(rename = "log")]
    pub log: DatabaseLogLevel,
    #[serde(rename = "min_conn")]
    pub min_conn: u32,
}
#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub enum DatabaseType {
    #[serde(rename = "postgres")]
    Postgres,
    #[serde(rename = "mysql")]
    MySql,
    #[serde(rename = "sqlite")]
    Sqlite,
}
#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub enum DatabaseLogLevel {
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "off")]
    Off,
}

impl AppDatabaseConfig {
    pub async fn conn(&self) -> DatabaseConnection {
        if self.db_type == DatabaseType::Sqlite {
            return Database::connect(self.db_url.clone())
                .await
                .expect("Failed to connect to database");
        }
        let mut config = ConnectOptions::new(&self.db_url.clone())
            .max_connections(self.max_conn)
            .min_connections(self.min_conn)
            .connect_lazy(self.lazy)
            .clone();
        match self.log {
            DatabaseLogLevel::Trace => {
                config = config
                    .sqlx_logging(true)
                    .sqlx_logging_level(log::LevelFilter::Trace)
                    .to_owned();
            }
            DatabaseLogLevel::Debug => {
                config = config
                    .sqlx_logging(true)
                    .sqlx_logging_level(log::LevelFilter::Debug)
                    .to_owned();
            }
            DatabaseLogLevel::Info => {
                config = config
                    .sqlx_logging(true)
                    .sqlx_logging_level(log::LevelFilter::Info)
                    .to_owned();
            }
            DatabaseLogLevel::Warn => {
                config = config
                    .sqlx_logging(true)
                    .sqlx_logging_level(log::LevelFilter::Warn)
                    .to_owned();
            }
            DatabaseLogLevel::Error => {
                config = config
                    .sqlx_logging(true)
                    .sqlx_logging_level(log::LevelFilter::Error)
                    .to_owned();
            }
            DatabaseLogLevel::Off => {}
        };
        Database::connect(config.clone())
            .await
            .expect("Failed to connect to database")
    }
}

impl Default for AppDatabaseConfig {
    fn default() -> Self {
        Self {
            db_type: DatabaseType::Sqlite,
            db_url: "...".to_string(),
            max_conn: 16,
            lazy: false,
            log: DatabaseLogLevel::Trace,
            min_conn: 2,
        }
    }
}
