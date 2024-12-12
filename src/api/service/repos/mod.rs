use sea_orm::DatabaseConnection;
use crate::metadata::transaction::repos::RepoTransaction;

pub mod create;
pub mod info;
pub mod owner;
pub mod commit;
pub mod branch;
pub mod object;
#[derive(Clone)]
pub struct RepoService{
    pub db: DatabaseConnection,
    pub transaction: RepoTransaction,
}