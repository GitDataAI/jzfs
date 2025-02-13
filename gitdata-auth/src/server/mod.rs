pub mod ctrl;
pub mod passwd;

use lib_config::AppNacos;
use lib_config::config::postgres::AppPostgresConfigKind;
use lib_entity::DatabaseConnection;

#[derive(Clone)]
pub struct AppAuthState {
    pub read : DatabaseConnection,
    pub write : DatabaseConnection,
}

impl AppAuthState {
    pub async fn init(nacos : AppNacos) -> anyhow::Result<Self> {
        let config = nacos.config;
        let read = config.postgres_connect(AppPostgresConfigKind::Read).await?;
        let write = config
            .postgres_connect(AppPostgresConfigKind::Write)
            .await?;
        Ok(Self { read, write })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() {
        let nacos = AppNacos::from_env().unwrap();
        let _ = AppAuthState::init(nacos).await.unwrap();
    }
}
