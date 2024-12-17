use sea_orm::DatabaseConnection;
use crate::metadata::service::MetaService;
use crate::server::email::EmailServer;

pub mod info;
pub mod create;
pub mod access;

#[allow(dead_code)]
pub struct RepoService{
    db: DatabaseConnection,
    redis: deadpool_redis::Pool,
    email: EmailServer,
}

impl From<&MetaService> for RepoService {
    fn from(value: &MetaService) -> Self {
        Self{
            db: value.pg(),
            redis: value.redis(),
            email: value.email(),
        }
    }
}