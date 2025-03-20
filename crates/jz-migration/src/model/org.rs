use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{ConnectionTrait, DbErr};
use sea_orm_migration::prelude::DeriveMigrationName;
use sea_orm_migration::{MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct OrgMigration;

#[async_trait]
impl MigrationTrait for OrgMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS team_member (
                uid UUID PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                description TEXT,
                group_uid UUID NOT NULL,
                team_uid UUID NOT NULL,
                access INT NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL
            );
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS team (
                uid UUID PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                group_uid UUID NOT NULL,
                secret BOOLEAN NOT NULL,
                access INT NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL
            );
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS members (
                uid UUID PRIMARY KEY,
                users_uid UUID NOT NULL,
                group_uid UUID NOT NULL,
                access INT NOT NULL,
                join_at TIMESTAMP NOT NULL
            );
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS invite (
                uid UUID PRIMARY KEY,
                org_uid UUID NOT NULL,
                team_uid UUID,
                user_uid UUID NOT NULL,
                email VARCHAR(255),
                access INT NOT NULL,
                status INT NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL
            );
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS groups (
                uid UUID PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                description TEXT,
                website TEXT,
                avatar TEXT,
                timezone TEXT,
                language TEXT,
                theme TEXT,
                location TEXT,
                topic TEXT[],
                setting TEXT[],
                active BOOLEAN NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                created_by UUID NOT NULL,
                owner_org TEXT
            );
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            
            "#,
        )
        .await?;
        Ok(())
    }
}
