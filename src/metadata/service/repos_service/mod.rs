use sea_orm::DatabaseConnection;
use crate::metadata::service::MetaService;
use crate::server::email::EmailServer;
use crate::server::mongodb::{MongoDBClient, MONGODB};

pub mod info;
pub mod create;
pub mod access;
pub mod blob;
pub mod sync;
pub mod branchs;
pub mod commits;
pub mod license;
pub mod readme;
pub mod files;
pub mod issues;
#[allow(dead_code)]
pub struct RepoService{
    db: DatabaseConnection,
    redis: deadpool_redis::Pool,
    email: EmailServer,
    mongo: MongoDBClient,
}

impl From<&MetaService> for RepoService {
    fn from(value: &MetaService) -> Self {
        let mongo = MONGODB.get().unwrap().clone();
        Self{
            db: value.pg(),
            redis: value.redis(),
            email: value.email(),
            mongo
        }
    }
}