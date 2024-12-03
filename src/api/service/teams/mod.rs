use sea_orm::DatabaseConnection;

pub mod list;
pub mod create;
pub mod invite;
pub mod byuser;
pub mod info;
#[derive(Clone)]
pub struct TeamService{
    pub(crate) db: DatabaseConnection,
}