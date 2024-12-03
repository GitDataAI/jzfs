use std::sync::RwLock;
use deadpool_redis::{Config, Connection, Pool, Runtime};
use log::info;
use tokio::sync::OnceCell;

pub static REDIS:OnceCell<RwLock<Connection>> = OnceCell::const_new();

#[derive(Clone)]
pub struct Redis{
    pub pool: Pool
}

impl Redis {
    pub async fn init() -> Redis {
        let redis_url = "redis://221.128.225.26:6379/0";
        info!("redis connect");

        let redis_cfg = Config::from_url(redis_url);
        let redis_pool = redis_cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        REDIS.get_or_init(|| async {
            RwLock::new(redis_pool.clone().get().await.unwrap())
        }).await;
        Self{
            pool: redis_pool
        }
    }
}