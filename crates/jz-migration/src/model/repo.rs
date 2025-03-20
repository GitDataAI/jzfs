use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{ConnectionTrait, DbErr, DeriveMigrationName};
use sea_orm_migration::{MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct RepoMigration;

#[async_trait]
impl MigrationTrait for RepoMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS watch (
                    uid UUID PRIMARY KEY,
                    user_id UUID NOT NULL,
                    repo_uid UUID NOT NULL,
                    level INT NOT NULL,
                    created_at TIMESTAMP NOT NULL
                );
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS commit (
                uid UUID PRIMARY KEY,
                id VARCHAR(255) NOT NULL,
                branch_uid UUID NOT NULL,
                repo_uid UUID NOT NULL,
                branch_name VARCHAR(255) NOT NULL,
                message TEXT NOT NULL,
                author VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                status VARCHAR(255) NOT NULL,
                runner TEXT[],
                time VARCHAR(255) NOT NULL
            );
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS branch (
                uid UUID PRIMARY KEY,
                repo_uid UUID NOT NULL,
                protect BOOLEAN NOT NULL,
                name VARCHAR(255) NOT NULL,
                head VARCHAR(255) NOT NULL,
                time VARCHAR(255) NOT NULL
            );
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS repository (
                uid UUID PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                description TEXT,
                owner_uid UUID NOT NULL,
                owner_name VARCHAR(255) NOT NULL,
                website VARCHAR(255),
                project UUID[],
                is_private BOOLEAN NOT NULL,
                fork UUID,
                default_branch VARCHAR(255) NOT NULL,
                nums_fork INT DEFAULT 0,
                nums_star INT DEFAULT 0,
                nums_watch INT DEFAULT 0,
                nums_issue INT DEFAULT 0,
                nums_pullrequest INT DEFAULT 0,
                nums_commit INT DEFAULT 0,
                nums_release INT DEFAULT 0,
                nums_tag INT DEFAULT 0,
                nums_branch INT DEFAULT 0,
                topic TEXT[],
                status VARCHAR(255) NOT NULL,
                rtype VARCHAR(255) NOT NULL,
                node UUID NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                created_by UUID NOT NULL
            );

            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
                 CREATE TABLE IF NOT EXISTS stars (
                    uid UUID PRIMARY KEY,
                    user_id UUID NOT NULL,
                    repository_id UUID NOT NULL,
                    created_at TIMESTAMP NOT NULL
                );
            "#,
        )
        .await?;
        Ok(())
    }
}
