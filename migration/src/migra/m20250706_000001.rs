use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
            CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS access_key (
            uid UUID PRIMARY KEY,
            title VARCHAR NOT NULL,
            description TEXT,
            name VARCHAR NOT NULL,
            token VARCHAR NOT NULL UNIQUE,
            access VARCHAR NOT NULL,
            use_history TEXT[],
            resource_owner VARCHAR NOT NULL,
            resource_owner_uid UUID NOT NULL,
            expiration VARCHAR NOT NULL,
            fingerprint VARCHAR NOT NULL,
            repo_access INT NOT NULL DEFAULT 0,
            email_access INT NOT NULL DEFAULT 0,
            event_access INT NOT NULL DEFAULT 0,
            gpg_access INT NOT NULL DEFAULT 0,
            ssh_access INT NOT NULL DEFAULT 0,
            webhook_access INT NOT NULL DEFAULT 0,
            wiki_access INT NOT NULL DEFAULT 0,
            project_access INT NOT NULL DEFAULT 0,
            issue_access INT NOT NULL DEFAULT 0,
            comment_access INT NOT NULL DEFAULT 0,
            profile_access INT NOT NULL DEFAULT 0,
            created_at TIMESTAMP NOT NULL DEFAULT now(),
            updated_at TIMESTAMP NOT NULL DEFAULT now()
        );
        "#,
        )
        .await?;
        Ok(())
    }
}
