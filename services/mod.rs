use std::{env, io};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use deadpool_redis::{Config, Connection, Pool, Runtime};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Statement};
use tokio::sync::OnceCell;
use tracing::info;
use crate::services::email::EmailEvent;
use crate::model::CREATE_TABLE;
use crate::services::product::post::DataProductPost;
use crate::services::repo::sync::RepoSync;

#[derive(Clone)]
pub struct AppState {
    pub read: DatabaseConnection,
    pub write: DatabaseConnection,
    pub email: EmailEvent,
    pub cache: Arc<Mutex<Connection>>,
}


pub mod auth;
pub mod user;
pub mod repo;
pub mod email;
pub mod recommend;
pub mod statistics;
pub mod access;
pub mod product;
impl AppState {
    pub async fn init_env() -> io::Result<AppState> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(20)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false);
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
        let pool = init_redis_store().await;
        let con = pool.get().await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let state = AppState {
            read,
            write,
            email: EmailEvent::new().await?,
            cache: Arc::new(Mutex::new(con))
        };
        DataProductPost::init(state.clone()).await;
        RepoSync::init(state.clone()).await;
        Ok(state)
    }
}
async fn init_redis_store() -> Pool {
    let cfg = Config::from_url(env::var("REDIS_URL").expect("REDIS_URL must be set"));
    cfg.create_pool(Some(Runtime::Tokio1)).expect("Failed to create Redis pool")
}


pub struct AppStateHandle;
static STATE:OnceCell<AppState> = OnceCell::const_new();
impl AppStateHandle {
    pub async fn get() -> AppState {
        STATE.get_or_init(|| async {
            AppState::init_env().await.unwrap()
        }).await.clone()
    }
}
