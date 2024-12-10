use sea_orm::DatabaseConnection;

pub mod apply;

#[derive(Clone)]
pub struct UserTransaction{
    pub db: DatabaseConnection
}