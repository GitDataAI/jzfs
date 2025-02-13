use lib_config::AppNacos;
use lib_config::config::postgres::AppPostgresConfigKind;
use lib_entity::DatabaseConnection;

use crate::transport::Transport;

#[derive(Clone)]
pub struct AppFsState {
    pub(crate) read : DatabaseConnection,
    pub(crate) write : DatabaseConnection,
    pub(crate) transport : Transport,
}

impl AppFsState {
    pub async fn init(nacos : AppNacos) -> anyhow::Result<Self> {
        let config = nacos.config;
        let read = config.postgres_connect(AppPostgresConfigKind::Read).await?;
        let write = config
            .postgres_connect(AppPostgresConfigKind::Write)
            .await?;
        Ok(Self {
            read,
            write,
            transport : Transport,
        })
    }
}

pub(crate) mod access;
pub(crate) mod avatar;
pub(crate) mod branch;
pub(crate) mod commit;
pub(crate) mod create;
pub(crate) mod ctrl;
pub(crate) mod delete;
pub(crate) mod info;
