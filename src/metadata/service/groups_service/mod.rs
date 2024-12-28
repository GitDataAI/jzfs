use sea_orm::DatabaseConnection;
use crate::metadata::service::MetaService;
use crate::server::email::EmailServer;

pub mod info;
pub mod create;
pub mod members;
pub mod check;
pub mod repos;
pub mod labels;

#[allow(dead_code)]
#[derive(Clone)]
pub struct GroupService{
    pub db: DatabaseConnection,
    pub redis: deadpool_redis::Pool,
    pub email: EmailServer,
}

impl From<&MetaService> for GroupService {
    fn from(value: &MetaService) -> Self {
        Self{
            db: value.pg(),
            redis: value.redis(),
            email: value.email(),
        }
    }
}