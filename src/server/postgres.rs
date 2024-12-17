use crate::config::CFG;
use sea_orm::*;
use std::time::Duration;
use tokio::sync::OnceCell;


pub static PGDB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn init_pg(){
    PGDB.get_or_init(||async {
        let cfg = CFG.get().unwrap().clone();
        let mut opt = ConnectOptions::new(cfg.postgres.format());
        opt.max_connections(cfg.postgres.max_connections)
            .min_connections(cfg.postgres.min_connections)
            .connect_timeout(Duration::from_secs(cfg.postgres.connect_timeout))
            .idle_timeout(Duration::from_secs(cfg.postgres.idle_timeout))
            .max_lifetime(Duration::from_secs(cfg.postgres.max_conn_lifetime))
            .sqlx_logging(true)
            .sqlx_logging_level(cfg.postgres.level())
            .set_schema_search_path(cfg.postgres.schema);
        Database::connect(opt).await.expect("Failed to connect to PostgreSQL")
    }).await;
}