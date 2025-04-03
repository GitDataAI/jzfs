use sea_orm::ConnectionTrait;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub mod org;
pub mod repo;
pub mod users;
pub mod utils;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(users::UserMigration),
            Box::new(repo::RepoMigration),
            Box::new(org::OrgMigration),
            Box::new(utils::UtilsMigration),
        ]
    }
}

pub async fn migrator() -> Result<(), sea_orm::DbErr> {
    let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not set");
    let migrator = sea_orm::Database::connect(&url)
        .await
        .expect("Failed to connect to database");
    migrator
        .execute_unprepared(r#"CREATE EXTENSION IF NOT EXISTS "uuid-ossp";"#)
        .await?;
    Migrator::up(&migrator, None).await?;
    migrator.close().await?;
    Ok(())
}
