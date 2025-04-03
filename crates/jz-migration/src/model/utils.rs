use sea_orm::{ConnectionTrait, DbErr, DeriveMigrationName};
use sea_orm::prelude::async_trait::async_trait;
use sea_orm_migration::SchemaManager;

#[derive(DeriveMigrationName)]
pub struct UtilsMigration;

#[async_trait]
impl sea_orm_migration::MigrationTrait for UtilsMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("
            CREATE TABLE IF NOT EXISTS notification (
                uid UUID PRIMARY KEY,
                user_uid UUID NOT NULL,
                repo_uid UUID,
                issue_uid UUID,
                comment_uid UUID,
                replay_user_uid UUID,
                title VARCHAR,
                content TEXT NOT NULL,
                read BOOLEAN NOT NULL,
                created_at TIMESTAMP NOT NULL
            );
        ").await?;
        
        Ok(())
    }
}