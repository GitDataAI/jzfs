use deadpool_redis::{Config, PoolConfig, Runtime};
use tokio::sync::OnceCell;
use crate::config::CFG;

pub static REDIS: OnceCell<deadpool_redis::Pool> = OnceCell::const_new();

pub async fn init_redis(){
    REDIS.get_or_init(||async {
        tracing::info!("Initializing Redis...");
        let c = CFG.get().unwrap().clone();
        let mut cfg = Config::from_url(c.redis.format());
        cfg.pool = Some(PoolConfig{
            max_size: c.redis.pool_size,
            timeouts: Default::default(),
            queue_mode: Default::default(),
        });
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        tracing::info!("Redis initialized.");
        pool
    }).await;
}