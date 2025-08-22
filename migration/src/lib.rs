use crate::sea_orm::DatabaseConnection;
pub use sea_orm_migration::prelude::*;

mod m20250812_000001_create_table;
mod m20250816_000002_add_is_private_to_git_repo;
mod m20250816_000003_remove_is_private_from_git_refs;
mod m20250816_000004_fix_git_commit_table;
mod m20250817_182246_add_name_email_to_user_repo_active;
mod m20250818_000005_create_git_repo_stats_table;
mod m20250818_000006_create_user_star_repo_table;
mod m20250818_000007_create_user_watch_repo_table;
mod m20250819_000008_update_recommendation_tables;
mod m20250819_000009_add_repo_stats_triggers;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250812_000001_create_table::Migration),
            Box::new(m20250816_000002_add_is_private_to_git_repo::Migration),
            Box::new(m20250816_000003_remove_is_private_from_git_refs::Migration),
            Box::new(m20250816_000004_fix_git_commit_table::Migration),
            Box::new(m20250817_182246_add_name_email_to_user_repo_active::Migration),
            Box::new(m20250818_000005_create_git_repo_stats_table::Migration),
            Box::new(m20250818_000006_create_user_star_repo_table::Migration),
            Box::new(m20250818_000007_create_user_watch_repo_table::Migration),
            Box::new(m20250819_000008_update_recommendation_tables::Migration),
            Box::new(m20250819_000009_add_repo_stats_triggers::Migration),
        ]
    }
}

pub async fn migration(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::down(db, None).await?;
    Migrator::up(db, None).await?;
    Ok(())
}
