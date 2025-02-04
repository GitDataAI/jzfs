use crate::config::AppConfig;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct AppPostgresConfig {
    pub kind: AppPostgresConfigKind,
    pub name: Option<String>,
    pub host: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub database: String,
    pub max_connection: i32,
    pub min_connection: i32,
}

#[derive(Deserialize,Serialize,Clone,Debug,PartialEq)]
pub enum AppPostgresConfigKind {
    Read,
    Write,
    Dead,
}

impl AppPostgresConfig {
    pub fn from_json(json: &str) -> std::io::Result<AppPostgresConfig> {
        let config: AppPostgresConfig = serde_json::from_str(json)?;
        Ok(config)
    }
    pub fn to_json(&self) -> std::io::Result<String> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }
    pub fn to_string(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.database)
    }
}

impl AppConfig {
    pub async fn postgres_config(&self, kind: AppPostgresConfigKind) -> std::io::Result<AppPostgresConfig> {
        let mut idx = 0;
        loop {
            let data_id = format!("postgres.{}", idx);
            idx += 1;
            if let Ok(data) = self.client.get_config(data_id, "postgres".to_string()).await {
                if let Ok(data) = serde_json::from_str::<AppPostgresConfig>(data.content()) {
                    if data.kind != kind {
                        continue;
                    } else {
                        break Ok(data);
                    }
                }
            } else {
                break Err(std::io::Error::new(std::io::ErrorKind::Other, "No postgres config found"));
            }
        }
    }
    pub async fn postgres_connect(&self, kind: AppPostgresConfigKind) -> std::io::Result<sea_orm::DatabaseConnection> {
        let config = self.postgres_config(kind).await?;
        let mut db_config = sea_orm::ConnectOptions::new(config.to_string());
        db_config
            .max_connections(config.max_connection as u32)
            .min_connections(config.min_connection as u32)
            .connect_lazy(true)
            .sqlx_logging(false);
        let client = sea_orm::Database::connect(db_config).await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::NetworkDown, e))?;
        Ok(client)
    }

}

impl Default for AppPostgresConfig {
    fn default() -> Self {
        AppPostgresConfig {
            kind: AppPostgresConfigKind::Dead,
            name: None,
            host: "127.0.0.1".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            password: "postgres".to_string(),
            database: "postgres".to_string(),
            max_connection: 10,
            min_connection: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::AppNacos;
    use super::*;

    #[test]
    fn test_app_postgres_config() {
        let config = AppPostgresConfig::default();
        let json = config.to_json().unwrap();
        println!("{}", json);
        let config = AppPostgresConfig::from_json(&json).unwrap();
        println!("{:?}", config);
    }
    #[tokio::test]
    async fn test_app_postgres_config_read() {
        let nacos = AppNacos::from_env().unwrap();
        let config = nacos.config.clone();
        let config = config.postgres_config(AppPostgresConfigKind::Read).await;
        assert!(config.is_ok());
        assert_eq!(config.unwrap().kind, AppPostgresConfigKind::Read);
    }
    #[tokio::test]
    async fn test_app_postgres_config_write() {
        let nacos = AppNacos::from_env().unwrap();
        let config = nacos.config.clone();
        let config = config.postgres_connect(AppPostgresConfigKind::Write).await;
        assert!(config.is_ok());
        assert!(config.unwrap().ping().await.is_ok());
    }
}
