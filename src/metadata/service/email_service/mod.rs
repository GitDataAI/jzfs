use sea_orm::DatabaseConnection;
use crate::metadata::service::MetaService;
use crate::server::email::EmailServer;

pub mod captcha;
pub mod forget;

#[allow(unused)]
pub struct EmailService{
    db: DatabaseConnection,
    redis: deadpool_redis::Pool,
    email: EmailServer,
}

impl From<&MetaService> for EmailService {
    fn from(value: &MetaService) -> Self {
        Self{
            db: value.pg(),
            redis: value.redis(),
            email: value.email(),
        }
    }
}