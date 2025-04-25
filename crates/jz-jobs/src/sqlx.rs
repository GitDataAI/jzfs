use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::{FromRow, Row};
use crate::Queue;

#[derive(Debug, FromRow, Clone)]
pub struct QueueModel {
    pub id: String,
    pub queue: String,
    pub data: String,
    pub status: String,
    pub created_unix: i64,
    pub updated_unix: i64,
}

#[derive(Clone)]
pub struct SqlxQueue {
    pub conn: sqlx::PgPool,
    pub table: String,
}

impl SqlxQueue {
    pub fn init(db: sqlx::PgPool, table: String) -> Self {
        Self { conn: db, table }
    }
}

#[async_trait::async_trait]
impl Queue for SqlxQueue {
    async fn push<T: Serialize + DeserializeOwned + Send + Sync>(&self, queue: &str, data: T) -> anyhow::Result<String> {
        let sql = format!("INSERT INTO {} (id, queue, data, status, created_unix, updated_unix) VALUES ($1, $2, $3, $4, $5, $6)", self.table);
        let id = uuid::Uuid::new_v4().to_string();
        let data = serde_json::to_string(&data)?;
        sqlx::query(&sql)
            .bind(id.clone())
            .bind(queue.to_string())
            .bind(data)
            .bind("pending".to_string())
            .bind(chrono::Utc::now().timestamp())
            .bind(chrono::Utc::now().timestamp())
            .execute(&self.conn)
            .await?;
        Ok(id)
    }

    async fn pull<T: Serialize + DeserializeOwned + Send + Sync>(&self, queue: &str) -> anyhow::Result<Option<(String, T)>> {
        let sql = format!("SELECT * FROM {} WHERE queue = $1 AND status = $2 ORDER BY created_unix ASC LIMIT 1 FOR UPDATE SKIP LOCKED", self.table);
        let row = sqlx::query_as::<_, QueueModel>(&sql)
            .bind(queue.to_string())
            .bind("pending".to_string())
            .fetch_optional(&self.conn)
            .await?;
        if let Some(row) = row {
            let data: T = serde_json::from_str(&row.data)?;
            sqlx::query(&format!("UPDATE {} SET status = $1, updated_unix = $2 WHERE id = $3", self.table))
                .bind("processing".to_string())
                .bind(chrono::Utc::now().timestamp())
                .bind(row.id.clone())
                .execute(&self.conn)
                .await?;
            Ok(Some((row.id, data)))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, queue: &str, id: String) -> anyhow::Result<()> {
        let sql = format!("DELETE FROM {} WHERE id = $1 AND queue = $2", self.table);
        sqlx::query(&sql)
            .bind(id)
            .bind(queue.to_string())
            .execute(&self.conn)
            .await?;
        Ok(())
    }

    async fn ok(&self, queue: &str, id: String) -> anyhow::Result<()> {
        let sql = format!("UPDATE {} SET status = $1, updated_unix = $2 WHERE id = $3 AND queue = $4", self.table);
        sqlx::query(&sql)
            .bind("ok".to_string())
            .bind(chrono::Utc::now().timestamp())
            .bind(id)
            .bind(queue.to_string())
            .execute(&self.conn)
            .await?;
        Ok(())
    }

    async fn fail(&self, queue: &str, id: String) -> anyhow::Result<()> {
        let sql = format!("UPDATE {} SET status = $1, updated_unix = $2 WHERE id = $3 AND queue = $4", self.table);
        sqlx::query(&sql)
            .bind("fail".to_string())
            .bind(chrono::Utc::now().timestamp())
            .bind(id)
            .bind(queue.to_string())
            .execute(&self.conn)
            .await?;
        Ok(())
    }

    async fn count(&self, queue: &str) -> anyhow::Result<usize> {
        let sql = format!("SELECT COUNT(*) FROM {} WHERE queue = $1 AND status = $2", self.table);
        let count = sqlx::query(&sql)
            .bind(queue.to_string())
            .bind("pending".to_string())
            .fetch_one(&self.conn)
            .await?
            .get::<i64, _>(0);
        Ok(count as usize)
    }

    async fn clear(&self, queue: &str) -> anyhow::Result<()> {
        let sql = format!("DELETE FROM {} WHERE queue = $1 AND status = $2", self.table);
        sqlx::query(&sql)
            .bind(queue.to_string())
            .bind("pending".to_string())
            .execute(&self.conn)
            .await?;
        Ok(())
    }

    async fn clear_all(&self) -> anyhow::Result<()> {
        let sql = format!("DELETE FROM {}", self.table);
        sqlx::query(&sql)
            .execute(&self.conn)
            .await?;
        Ok(())
    }

    async fn init(&self) -> anyhow::Result<()> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id VARCHAR(255) PRIMARY KEY,
                queue VARCHAR(255) NOT NULL,
                data TEXT NOT NULL,
                status VARCHAR(255) NOT NULL,
                created_unix BIGINT NOT NULL,
                updated_unix BIGINT NOT NULL
            )",
            self.table
        );
        sqlx::query(&sql)
            .execute(&self.conn)
            .await?;
        Ok(())
    }
}

impl SqlxQueue {
    pub async fn pulls<T: Serialize + DeserializeOwned + Send + Sync>(&self, queue: &str, limit: usize) -> anyhow::Result<Vec<(String, T)>> {
        let sql = format!(
            "SELECT * FROM {} WHERE queue = '{}' AND status = 'wait' ORDER BY created_unix LIMIT {};",
            self.table, queue, limit
        );
        let sql = sqlx::query_as::<_, QueueModel>(&sql)
            .fetch_all(&self.conn)
            .await?;
        Ok(sql
            .into_iter()
            .map(|sql| (sql.id, serde_json::from_str(&sql.data).unwrap()))
            .collect())
    }
}
