use sea_orm::DatabaseConnection;

pub mod create;
pub mod info;
pub mod update;
pub mod member;

#[derive(Clone)]
pub struct GroupService{
    pub(crate) db: DatabaseConnection
}