use sea_orm::DatabaseConnection;
use crate::metadata::service::MetaService;
use crate::server::email::EmailServer;

pub mod applications;
pub mod avatar;
pub mod blocks;
pub mod emails;
pub mod follower;
pub mod gpg_keys;
pub mod hooks;
pub mod keys;
pub mod repos;
pub mod starred;
pub mod subscriptions;
pub mod login;
pub mod apply;
pub mod reset;
pub mod info;
pub mod check;
pub mod setting;
pub mod search;


#[derive(Clone)]
pub struct UserService{
    pub db: DatabaseConnection,
    pub redis: deadpool_redis::Pool,
    pub email: EmailServer,
}


impl From<&MetaService> for UserService {
    fn from(value: &MetaService) -> Self {
        Self{
            db: value.pg(),
            redis: value.redis(),
            email: value.email(),
        }
    }
}