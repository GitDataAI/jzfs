use sea_orm::DatabaseConnection;

pub mod create;
pub mod sync;
pub mod object_tree;


#[derive(Clone)]
pub struct RepoTransaction{
    db: DatabaseConnection
}

impl RepoTransaction {
    pub fn new(db: DatabaseConnection) -> Self{
        RepoTransaction{
            db
        }
    }
}