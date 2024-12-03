use sea_orm::DatabaseConnection;

pub mod create;
pub mod info;

#[derive(Clone)]
pub struct GroupService{
    pub(crate) db: DatabaseConnection
}