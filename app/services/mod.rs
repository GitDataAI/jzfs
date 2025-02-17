use std::{env, io};
use std::time::Duration;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Statement};
use tracing::info;
use crate::app::services::email::EmailEvent;
use crate::model::CREATE_TABLE;

#[derive(Clone)]
pub struct AppState {
    pub read: DatabaseConnection,
    pub write: DatabaseConnection,
    pub email: EmailEvent,
}


pub mod auth;
pub mod user;
pub mod repo;
pub mod email;

impl AppState {
    pub async fn init_env() -> io::Result<AppState> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);
        let read = Database::connect(opt.clone()).await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        info!("Connected to database for read");
        let write = Database::connect(opt.clone()).await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        match read.execute(Statement::from_string(DatabaseBackend::Postgres, "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";"))
            .await{
            Ok(x) => {
                info!("Enable Uuid success");
                info!("exec result rows_affected: {}",x.rows_affected())
            },
            Err(e) => {
                info!("Create table failed: {}", e);
            }
        }
        let sql = CREATE_TABLE
            .split(";")
            .map(|x|x.trim())
            .filter(|x|!x.is_empty())
            .collect::<Vec<_>>();
        for sql in sql {
            match write.execute(Statement::from_string(DatabaseBackend::Postgres, sql))
                .await{
                Ok(x) => {
                    info!("Create table success");
                    info!("exec result rows_affected: {}",x.rows_affected())
                },
                Err(e) => {
                    info!("Create table failed: {}", e);
                }
            }
        }
        info!("Connected to database for write");
        
        Ok(AppState {
            read,
            write,
            email: EmailEvent::new().await?,
        })
    }
}

