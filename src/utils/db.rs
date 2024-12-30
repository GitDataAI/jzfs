use crate::config::CFG;
use crate::emails::Email;
use crate::init_repo_dir;
use crate::server::MetaData;
use deadpool_redis::{Config, PoolConfig, Runtime};
use sea_orm::*;
use std::time::Duration;
use tokio::sync::OnceCell;

pub static PGDB: OnceCell<DatabaseConnection> = OnceCell::const_new();
pub static REDIS: OnceCell<deadpool_redis::Pool> = OnceCell::const_new();
pub static EMAIL_SERVICE: OnceCell<Email> = OnceCell::const_new();
#[allow(non_snake_case)]
pub async fn Init() {
    init_repo_dir().unwrap();
    init_pg().await;
    init_redis().await;
    init_email().await;
    MetaData::init().await;
}
pub async fn init_pg() {
    PGDB.get_or_init(|| async {
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
        Database::connect(opt)
            .await
            .expect("Failed to connect to PostgreSQL")
    })
    .await;
}

pub async fn init_email() {
    EMAIL_SERVICE
        .get_or_init(|| async { Email::init().await })
        .await;
}

pub async fn init_redis() {
    REDIS
        .get_or_init(|| async {
            tracing::info!("Initializing Redis...");
            let c = CFG.get().unwrap().clone();
            let mut cfg = Config::from_url(c.redis.format());
            cfg.pool = Some(PoolConfig {
                max_size: c.redis.pool_size,
                timeouts: Default::default(),
                queue_mode: Default::default(),
            });
            let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
            tracing::info!("Redis initialized.");
            pool
        })
        .await;
}

#[allow(non_snake_case)]
pub async fn Postgres() -> DatabaseConnection {
    PGDB.get().unwrap().clone()
}
#[allow(non_snake_case)]
pub async fn Redis() -> deadpool_redis::Pool {
    REDIS.get().unwrap().clone()
}
