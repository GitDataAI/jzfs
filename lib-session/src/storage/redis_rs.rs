
use std::sync::Arc;
use anyhow::Error;
use redis::{aio::ConnectionManager,AsyncCommands, Client, Cmd, FromRedisValue, Value};

use super::SessionKey;
use crate::storage::{
    interface::{LoadError, SaveError, SessionState, UpdateError},
    utils::generate_session_key,
    SessionStore,
};
#[derive(Clone)]
pub struct RedisSessionStore {
    configuration: CacheConfiguration,
    client: RedisSessionConn,
}

#[derive(Clone)]
enum RedisSessionConn {
    /// Single connection.
    Single(ConnectionManager),

    /// Connection pool.
    #[cfg(feature = "redis-pool")]
    Pool(deadpool_redis::Pool),
}

#[derive(Clone)]
struct CacheConfiguration {
    cache_keygen: Arc<dyn Fn(&str) -> String + Send + Sync>,
}

impl Default for CacheConfiguration {
    fn default() -> Self {
        Self {
            cache_keygen: Arc::new(str::to_owned),
        }
    }
}

impl RedisSessionStore {
    pub fn builder(connection_string: impl Into<String>) -> RedisSessionStoreBuilder {
        RedisSessionStoreBuilder {
            configuration: CacheConfiguration::default(),
            conn_builder: RedisSessionConnBuilder::Single(connection_string.into()),
        }
    }
    
    #[cfg(feature = "redis-pool")]
    pub fn builder_pooled(pool: impl Into<deadpool_redis::Pool>) -> RedisSessionStoreBuilder {
        RedisSessionStoreBuilder {
            configuration: CacheConfiguration::default(),
            conn_builder: RedisSessionConnBuilder::Pool(pool.into()),
        }
    }
    
    pub async fn new(connection_string: impl Into<String>) -> Result<RedisSessionStore, Error> {
        Self::builder(connection_string).build().await
    }
    
    #[cfg(feature = "redis-pool")]
    pub async fn new_pooled(
        pool: impl Into<deadpool_redis::Pool>,
    ) -> anyhow::Result<RedisSessionStore> {
        Self::builder_pooled(pool).build().await
    }
}

#[must_use]
pub struct RedisSessionStoreBuilder {
    configuration: CacheConfiguration,
    conn_builder: RedisSessionConnBuilder,
}

enum RedisSessionConnBuilder {
    /// Single connection string.
    Single(String),

    /// Pre-built connection pool.
    #[cfg(feature = "redis-pool")]
    Pool(deadpool_redis::Pool),
}

impl RedisSessionConnBuilder {
    async fn into_client(self) -> anyhow::Result<RedisSessionConn> {
        Ok(match self {
            RedisSessionConnBuilder::Single(conn_string) => {
                RedisSessionConn::Single(ConnectionManager::new(Client::open(conn_string)?).await?)
            }

            #[cfg(feature = "redis-pool")]
            RedisSessionConnBuilder::Pool(pool) => RedisSessionConn::Pool(pool),
        })
    }
}

impl RedisSessionStoreBuilder {
    /// Set a custom cache key generation strategy, expecting a session key as input.
    pub fn cache_keygen<F>(mut self, keygen: F) -> Self
    where
        F: Fn(&str) -> String + 'static + Send + Sync,
    {
        self.configuration.cache_keygen = Arc::new(keygen);
        self
    }

    /// Finalises builder and returns a [`RedisSessionStore`] instance.
    pub async fn build(self) -> anyhow::Result<RedisSessionStore> {
        let client = self.conn_builder.into_client().await?;

        Ok(RedisSessionStore {
            configuration: self.configuration,
            client,
        })
    }
}


impl RedisSessionStore {
    /// Execute Redis command and retry once in certain cases.
    ///
    /// `ConnectionManager` automatically reconnects when it encounters an error talking to Redis.
    /// The request that bumped into the error, though, fails.
    ///
    /// This is generally OK, but there is an unpleasant edge case: Redis client timeouts. The
    /// server is configured to drop connections who have been active longer than a pre-determined
    /// threshold. `redis-rs` does not proactively detect that the connection has been dropped - you
    /// only find out when you try to use it.
    ///
    /// This helper method catches this case (`.is_connection_dropped`) to execute a retry. The
    /// retry will be executed on a fresh connection, therefore it is likely to succeed (or fail for
    /// a different more meaningful reason).
    #[allow(clippy::needless_pass_by_ref_mut)]
    async fn execute_command<T: FromRedisValue>(&self, cmd: &mut Cmd) -> anyhow::Result<T> {
        let mut can_retry = true;

        match self.client {
            RedisSessionConn::Single(ref conn) => {
                let mut conn = conn.clone();

                loop {
                    match cmd.query_async(&mut conn).await {
                        Ok(value) => return Ok(value),
                        Err(err) => {
                            if can_retry && err.is_connection_dropped() {
                                tracing::debug!(
                                    "Connection dropped while trying to talk to Redis. Retrying."
                                );

                                // Retry at most once
                                can_retry = false;

                                continue;
                            } else {
                                return Err(err.into());
                            }
                        }
                    }
                }
            }

            #[cfg(feature = "redis-pool")]
            RedisSessionConn::Pool(ref pool) => {
                let mut conn = pool.get().await?;

                loop {
                    match cmd.query_async(&mut conn).await {
                        Ok(value) => return Ok(value),
                        Err(err) => {
                            if can_retry && err.is_connection_dropped() {
                                tracing::debug!(
                                    "Connection dropped while trying to talk to Redis. Retrying."
                                );

                                // Retry at most once
                                can_retry = false;

                                continue;
                            } else {
                                return Err(err.into());
                            }
                        }
                    }
                }
            }
        }
    }
}
