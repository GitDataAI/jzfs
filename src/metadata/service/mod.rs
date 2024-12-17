use sea_orm::DatabaseConnection;
use crate::metadata::service::email_service::EmailService;
use crate::metadata::service::groups_service::GroupService;
use crate::metadata::service::repos_service::RepoService;
use crate::metadata::service::users_service::UserService;
use crate::server::{Postgres, Redis};
use crate::server::email::{EmailServer, EMAIL_SERVICE};


pub mod users_service;
pub mod repos_service;
pub mod groups_service;
pub mod email_service;
#[derive(Clone)]
pub struct MetaService{
    pg: DatabaseConnection,
    redis: deadpool_redis::Pool,
    email: EmailServer,
}

impl MetaService{
    pub fn pg(&self) -> DatabaseConnection{
        self.pg.clone()
    }

    pub fn redis(&self) -> deadpool_redis::Pool{
        self.redis.clone()
    }

    pub fn email(&self) -> EmailServer{
        self.email.clone()
    }
}

pub static META: tokio::sync::OnceCell<MetaService> = tokio::sync::OnceCell::const_new();


impl MetaService{
    pub async fn init() -> MetaService {
        let email = EMAIL_SERVICE.get().unwrap().clone();
        let once = Self{
            pg: Postgres().await,
            redis: Redis().await,
            email,
        };
        META.get_or_init(||async { 
            once.clone()
        }).await.clone()
    }
    pub fn user_service(&self) -> UserService{
        UserService::from(self)
    }
    pub fn repo_service(&self) -> RepoService {
        RepoService::from(self)
    }
    pub fn group_service(&self) -> GroupService {
        GroupService::from(self)
    }
    pub fn email_service(&self) -> EmailService {
        EmailService::from(self)
    }
}