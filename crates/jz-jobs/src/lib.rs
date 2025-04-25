use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[cfg(feature = "sql")]
pub mod seaorm;
#[cfg(feature = "sql")]
pub use seaorm::*;
use crate::sqlx::SqlxQueue;

pub mod sqlx;

#[async_trait]
pub trait Queue: Sync + Send + 'static + Clone + Sized {
    async fn push<T: serde::Serialize + serde::de::DeserializeOwned + Send + Sync>(
        &self,
        queue: &str,
        data: T,
    ) -> anyhow::Result<String>;
    async fn pull<T: serde::Serialize + serde::de::DeserializeOwned + Send + Sync>(
        &self,
        queue: &str,
    ) -> anyhow::Result<Option<(String, T)>>;
    async fn delete(&self, queue: &str, id: String) -> anyhow::Result<()>;
    async fn ok(&self, queue: &str, id: String) -> anyhow::Result<()>;
    async fn fail(&self, queue: &str, id: String) -> anyhow::Result<()>;
    async fn count(&self, queue: &str) -> anyhow::Result<usize>;
    async fn clear(&self, queue: &str) -> anyhow::Result<()>;
    async fn clear_all(&self) -> anyhow::Result<()>;
    async fn init(&self) -> anyhow::Result<()>;
}


#[derive(Clone)]
pub enum QueueJobs {
    Seaorm(SeaOrmQueue),
    Sqlx(sqlx::SqlxQueue),
}

impl QueueJobs {
    pub fn new_sqlx(sqlx: SqlxQueue) -> QueueJobs {
        QueueJobs::Sqlx(sqlx)
    }
    pub fn new_seaorm(seaorm: SeaOrmQueue) -> QueueJobs {
        QueueJobs::Seaorm(seaorm)
    }
}

#[async_trait]
impl Queue for QueueJobs {
    async fn push<T: Serialize + DeserializeOwned + Send + Sync>(&self, q: &str, data: T) -> anyhow::Result<String> {
        match self {
            QueueJobs::Seaorm(queue) => queue.push(q, data).await,
            QueueJobs::Sqlx(queue) => queue.push(q, data).await,
        }
    }

    async fn pull<T: Serialize + DeserializeOwned + Send + Sync>(&self, q: &str) -> anyhow::Result<Option<(String, T)>> {
        match self {
            QueueJobs::Seaorm(queue) => queue.pull(q).await,
            QueueJobs::Sqlx(queue) => queue.pull(q).await,
        }
    }

    async fn delete(&self, q: &str, id: String) -> anyhow::Result<()> {
        match self {
            QueueJobs::Seaorm(queue) => queue.delete(q, id).await,
            QueueJobs::Sqlx(queue) => queue.delete(q, id).await,
        }
    }

    async fn ok(&self, q: &str, id: String) -> anyhow::Result<()> {
        match self {
            QueueJobs::Seaorm(queue) => queue.ok(q, id).await,
            QueueJobs::Sqlx(queue) => queue.ok(q, id).await,
        }
    }

    async fn fail(&self, q: &str, id: String) -> anyhow::Result<()> {
        match self {
            QueueJobs::Seaorm(queue) => queue.fail(q, id).await,
            QueueJobs::Sqlx(queue) => queue.fail(q, id).await,
        }
    }

    async fn count(&self, q: &str) -> anyhow::Result<usize> {
        match self {
            QueueJobs::Seaorm(queue) => queue.count(q).await,
            QueueJobs::Sqlx(queue) => queue.count(q).await,
        }
    }

    async fn clear(&self, q: &str) -> anyhow::Result<()> {
        match self {
            QueueJobs::Seaorm(queue) => queue.clear(q).await,
            QueueJobs::Sqlx(queue) => queue.clear(q).await,
        }   
    }

    async fn clear_all(&self) -> anyhow::Result<()> {
        match self {
            QueueJobs::Seaorm(queue) => queue.clear_all().await,
            QueueJobs::Sqlx(queue) => queue.clear_all().await,
        }
    }

    async fn init(&self) -> anyhow::Result<()> {
        match self {
            QueueJobs::Seaorm(queue) => queue.init().await,
            QueueJobs::Sqlx(queue) => queue.init().await,
        }
    }
}

impl QueueJobs {
    pub async fn pulls<T: Serialize + DeserializeOwned + Send + Sync>(&self, q: &str, limit: usize) -> anyhow::Result<Vec<(String, T)>> {
        match self {
            QueueJobs::Seaorm(queue) => queue.pulls(q, limit).await,
            QueueJobs::Sqlx(queue) => queue.pulls(q, limit).await,
        }
    }
}