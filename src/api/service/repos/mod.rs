use sea_orm::DatabaseConnection;
use crate::metadata::transaction::repos::RepoTransaction;

pub mod create;
pub mod info;
#[derive(Clone)]
pub struct RepoService{
    pub db: DatabaseConnection,
    pub transaction: RepoTransaction,
}