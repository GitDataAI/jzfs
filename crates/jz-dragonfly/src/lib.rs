
pub struct Dragonfly;
pub use deadpool_redis::*;

impl Dragonfly {
    pub fn env_url() -> String {
        std::env::var("REDIS_URL").unwrap()
    }
    pub fn connect_pool() -> Pool {
        let mut config = Config::from_url(Self::env_url());
        config.pool = Some(PoolConfig::new(512));
        let pool = config.create_pool(Some(Runtime::Tokio1)).expect("Failed to create redis pool");
        pool
    }
}