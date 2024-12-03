use sea_orm::DatabaseConnection;

pub mod login;
pub mod apply;
pub mod check;
pub mod reset;
pub mod update;
pub mod info;
#[derive(Clone)]
pub struct UserService{
    pub(crate) db: DatabaseConnection
}

