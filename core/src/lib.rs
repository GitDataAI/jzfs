use crate::email::email_thread::EmailThread;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use config::AppConfig;
use error::AppError;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct AppCore {
    pub db: DatabaseConnection,
    pub config: AppConfig,
    pub redis: Pool<RedisConnectionManager>,
}

pub mod activities;
pub mod auth;
pub mod comments;
pub mod email;
pub mod issues;
pub mod projects;
pub mod pull_requests;
pub mod reactions;
pub mod repos;
pub mod settings;
pub mod users;
pub mod wikis;

impl AppCore {
    pub async fn init_service(&self) -> Result<(), AppError> {
        EmailThread::init(self.config.email.clone()).await;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Paginator {
    pub page: u64,
    pub page_size: u64,
}
