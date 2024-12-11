use sea_orm::DatabaseConnection;

pub mod name;
pub mod exits;
pub mod session;
pub mod grand;

#[derive(Clone)]
pub struct CheckService{
    pub(crate) db: DatabaseConnection
}