use crate::Queue;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ConnectionTrait, DatabaseConnection, FromQueryResult, Statement};
use serde::Serialize;
use serde::de::DeserializeOwned;
use uuid::Uuid;

#[derive(Clone)]
pub struct SeaOrmQueue {
    pub conn: DatabaseConnection,
    pub table: String,
}

impl SeaOrmQueue {
    pub fn new(db: DatabaseConnection, table: String) -> Self {
        Self { conn: db, table }
    }
}

#[async_trait]
impl Queue for SeaOrmQueue {
    async fn push<T: Serialize + DeserializeOwned + Send + Sync>(
        &self,
        queue: &str,
        data: T,
    ) -> anyhow::Result<String> {
        let id = Uuid::new_v4().to_string();
        let sql = format!(
            "INSERT INTO {} (id, queue, data, status, created_unix, updated_unix) VALUES ('{}', '{}', '{}', 'wait', {}, {});",
            self.table,
            id,
            queue,
            serde_json::to_string(&data)?,
            Utc::now().naive_utc().and_utc().timestamp(),
            Utc::now().naive_utc().and_utc().timestamp()
        );
        let _ = self
            .conn
            .execute(Statement::from_string(
                self.conn.get_database_backend(),
                sql,
            ))
            .await?;
        Ok(id)
    }

    async fn pull<T: Serialize + DeserializeOwned + Send + Sync>(
        &self,
        queue: &str,
    ) -> anyhow::Result<Option<(String, T)>> {
        let sql = format!(
            "SELECT * FROM {} WHERE queue = '{}' AND status = 'wait' ORDER BY created_unix LIMIT 1;",
            self.table, queue
        );
        let sql = QueueModel::find_by_statement(Statement::from_sql_and_values(
            self.conn.get_database_backend(),
            sql,
            [],
        ))
        .one(&self.conn)
        .await?;
        if sql.is_none() {
            return Ok(None);
        }
        let sql = sql.unwrap();
        Ok(Some((
            sql.id,
            serde_json::from_str(&sql.data)?,
        )))
    }

    async fn delete(&self, queue: &str, id: String) -> anyhow::Result<()> {
        let sql = format!(
            "DELETE FROM {} WHERE id = '{}' AND queue = '{}';",
            self.table, id, queue
        );
        self.conn
            .execute(Statement::from_string(
                self.conn.get_database_backend(),
                sql,
            ))
            .await?;
        Ok(())
    }

    async fn ok(&self, queue: &str, id: String) -> anyhow::Result<()> {
        let sql = format!(
            "UPDATE {} SET status = 'ok', updated_unix = {} WHERE id = '{}' AND queue = '{}';",
            self.table,
            Utc::now().naive_utc().and_utc().timestamp(),
            id,
            queue
        );
        self.conn
            .execute(Statement::from_string(
                self.conn.get_database_backend(),
                sql,
            ))
            .await?;
        Ok(())
    }

    async fn fail(&self, queue: &str, id: String) -> anyhow::Result<()> {
        let sql = format!(
            "UPDATE {} SET status = 'fail', updated_unix = {} WHERE id = '{}' AND queue = '{}';",
            self.table,
            Utc::now().naive_utc().and_utc().timestamp(),
            id,
            queue
        );
        self.conn
            .execute(Statement::from_string(
                self.conn.get_database_backend(),
                sql,
            ))
            .await?;
        Ok(())
    }

    async fn count(&self, queue: &str) -> anyhow::Result<usize> {
        let sql = format!(
            "SELECT COUNT(*) as count FROM {} WHERE queue = '{}';",
            self.table, queue
        );
        let result = self
            .conn
            .query_one(Statement::from_string(
                self.conn.get_database_backend(),
                sql,
            ))
            .await?
            .ok_or(anyhow::anyhow!("count error"))?;
        let count = result.try_get::<i64>("", "count")?;
        Ok(count as usize)
    }

    async fn clear(&self, queue: &str) -> anyhow::Result<()> {
        let sql = format!("DELETE FROM {} WHERE queue = '{}';", self.table, queue);
        self.conn
            .execute(Statement::from_string(
                self.conn.get_database_backend(),
                sql,
            ))
            .await?;
        Ok(())
    }

    async fn clear_all(&self) -> anyhow::Result<()> {
        let sql = format!("DROP TABLE IF EXISTS {};", self.table);
        self.conn
            .execute(Statement::from_string(
                self.conn.get_database_backend(),
                sql,
            ))
            .await?;
        Ok(())
    }
    async fn init(&self) -> anyhow::Result<()> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id TEXT PRIMARY KEY,
            queue TEXT NOT NULL,
            data TEXT NOT NULL,
            status TEXT NOT NULL,
            created_unix bigint NOT NULL,
            updated_unix bigint NOT NULL
        );",
            self.table
        );
        self.conn
            .execute(Statement::from_string(
                self.conn.get_database_backend(),
                sql,
            ))
            .await?;
        Ok(())
    }
}

impl SeaOrmQueue {
    pub async fn pulls<T: Serialize + DeserializeOwned + Send + Sync>(&self, queue: &str, limit: usize) -> anyhow::Result<Vec<(String, T)>> {
        let sql = format!(
            "SELECT * FROM {} WHERE queue = '{}' AND status = 'wait' ORDER BY created_unix LIMIT {};",
            self.table, queue, limit
        );
        let sql = QueueModel::find_by_statement(Statement::from_sql_and_values(
            self.conn.get_database_backend(),
            sql,
            [],
        ))
        .all(&self.conn)
        .await?;
        Ok(sql
            .into_iter()
            .map(|sql| (sql.id, serde_json::from_str(&sql.data).unwrap()))
            .collect())
    }
}
#[derive(Debug, FromQueryResult, Clone)]
pub struct QueueModel {
    pub id: String,
    pub queue: String,
    pub data: String,
    pub status: String,
    pub created_unix: i64,
    pub updated_unix: i64,
}

#[cfg(test)]
mod tests {
    use crate::Queue;
    use crate::seaorm::SeaOrmQueue;
    use sea_orm::DatabaseConnection;
    use serde::{Deserialize, Serialize};

    async fn init_sqlite() -> DatabaseConnection {
        sea_orm::Database::connect("sqlite::memory:").await.unwrap()
    }
    #[tokio::test]
    async fn test_init() -> anyhow::Result<()> {
        let conn = init_sqlite().await;
        let queue = SeaOrmQueue {
            conn,
            table: "test_queue".to_string(),
        };
        assert!(queue.init().await.is_ok());
        Ok(())
    }
    #[derive(Deserialize, Serialize, Clone, Debug)]
    struct TestData {
        id: String,
        name: String,
    }
    #[tokio::test]
    async fn test_push() -> anyhow::Result<()> {
        let conn = init_sqlite().await;
        let queue = SeaOrmQueue {
            conn,
            table: "test_queue".to_string(),
        };
        assert!(queue.init().await.is_ok());
        assert!(
            queue
                .push("test", TestData {
                    id: "1".to_string(),
                    name: "test".to_string()
                })
                .await
                .is_ok()
        );
        Ok(())
    }
    #[tokio::test]
    async fn test_pull() -> anyhow::Result<()> {
        let conn = init_sqlite().await;
        let queue = SeaOrmQueue {
            conn,
            table: "test_queue".to_string(),
        };
        assert!(queue.init().await.is_ok());
        assert!(
            queue
                .push("test", TestData {
                    id: "1".to_string(),
                    name: "test".to_string()
                })
                .await
                .is_ok()
        );
        let data = queue.pull::<TestData>("test").await;
        assert!(data?.is_some());
        Ok(())
    }
    #[tokio::test]
    async fn test_delete() -> anyhow::Result<()> {
        let conn = init_sqlite().await;
        let queue = SeaOrmQueue {
            conn,
            table: "test_queue".to_string(),
        };
        assert!(queue.init().await.is_ok());
        assert!(
            queue
                .push("test", TestData {
                    id: "1".to_string(),
                    name: "test".to_string()
                })
                .await
                .is_ok()
        );
        let data = queue.pull::<TestData>("test").await;
        assert!(data.is_ok());
        let data = data?.unwrap();
        assert!(queue.delete("test", data.clone().0).await.is_ok());
        Ok(())
    }
    #[tokio::test]
    async fn test_ok() -> anyhow::Result<()> {
        let conn = init_sqlite().await;
        let queue = SeaOrmQueue {
            conn,
            table: "test_queue".to_string(),
        };
        assert!(queue.init().await.is_ok());
        assert!(
            queue
                .push("test", TestData {
                    id: "1".to_string(),
                    name: "test".to_string()
                })
                .await
                .is_ok()
        );
        let data = queue.pull::<TestData>("test").await;
        assert!(queue.ok("test", data?.unwrap().0).await.is_ok());
        Ok(())
    }
    #[tokio::test]
    async fn test_fail() -> anyhow::Result<()> {
        let conn = init_sqlite().await;
        let queue = SeaOrmQueue {
            conn,
            table: "test_queue".to_string(),
        };
        assert!(queue.init().await.is_ok());
        assert!(
            queue
                .push("test", TestData {
                    id: "1".to_string(),
                    name: "test".to_string()
                })
                .await
                .is_ok()
        );
        let data = queue.pull::<TestData>("test").await;
        assert!(queue.fail("test", data?.unwrap().0).await.is_ok());
        Ok(())
    }
    #[tokio::test]
    async fn test_count() -> anyhow::Result<()> {
        let conn = init_sqlite().await;
        let queue = SeaOrmQueue {
            conn,
            table: "test_queue".to_string(),
        };
        assert!(queue.init().await.is_ok());
        assert!(
            queue
                .push("test", TestData {
                    id: "1".to_string(),
                    name: "test".to_string()
                })
                .await
                .is_ok()
        );
        assert_eq!(queue.count("test").await?, 1);
        Ok(())
    }
    #[tokio::test]
    async fn test_clear() -> anyhow::Result<()> {
        let conn = init_sqlite().await;
        let queue = SeaOrmQueue {
            conn,
            table: "test_queue".to_string(),
        };
        assert!(queue.init().await.is_ok());
        assert!(
            queue
                .push("test", TestData {
                    id: "1".to_string(),
                    name: "test".to_string()
                })
                .await
                .is_ok()
        );
        assert!(queue.clear("test").await.is_ok());
        Ok(())
    }
}
