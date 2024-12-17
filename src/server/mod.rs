use sea_orm::DatabaseConnection;
use crate::metadata::service::MetaService;
use crate::server::email::init_email;
use crate::server::postgres::PGDB;

pub mod postgres;
pub mod sqlite;
// 保留, 未来可能启用
pub mod mysql;
// 保留, 未来可能启用
pub mod mongodb;
pub mod redis;
pub mod email;

#[allow(non_snake_case)]
pub async fn Init(){
    postgres::init_pg().await;
    redis::init_redis().await;
    sqlite::init_sqlite().await;
    init_email().await;
    MetaService::init().await;
}

#[allow(non_snake_case)]
pub async fn Postgres() -> DatabaseConnection{
    PGDB.get().unwrap().clone()
}
#[allow(non_snake_case)]
pub async fn Sqlite() -> DatabaseConnection{
    sqlite::SQLITEDB.get().unwrap().clone()
}
 #[allow(non_snake_case)]
 pub async fn Redis() -> deadpool_redis::Pool{
    redis::REDIS.get().unwrap().clone()
}