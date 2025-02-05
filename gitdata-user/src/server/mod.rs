
use lib_config::AppNacos;
use lib_config::config::postgres::AppPostgresConfigKind;
use lib_entity::DatabaseConnection;

#[derive(Clone)]
pub struct AppUserState {
    pub read: DatabaseConnection,
    pub write: DatabaseConnection,
}

impl AppUserState {
    pub async fn init(nacos: AppNacos) -> anyhow::Result<Self> {
        let config = nacos.config;
        let read = config.postgres_connect(AppPostgresConfigKind::Read).await?;
        let write = config.postgres_connect(AppPostgresConfigKind::Write).await?;
        Ok(Self {
            read,
            write,
        })
    }
}


pub mod optional;
pub mod email;
pub mod follow;