use sea_orm::DatabaseConnection;

pub mod list;
pub mod create;
pub mod invite;
#[derive(Clone)]
pub struct TeamService{
    pub(crate) db: DatabaseConnection,
}