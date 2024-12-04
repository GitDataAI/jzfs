use sea_orm::DatabaseConnection;

pub mod create;


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