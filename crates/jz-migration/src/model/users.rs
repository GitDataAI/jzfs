use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{ConnectionTrait, DbErr};
use sea_orm_migration::prelude::DeriveMigrationName;
use sea_orm_migration::{MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct UserMigration;

#[async_trait]
impl MigrationTrait for UserMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
          CREATE TABLE IF NOT EXISTS users (
                uid UUID PRIMARY KEY,
                username VARCHAR(255) NOT NULL,
                password VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL,
                description TEXT,
                avatar TEXT,
                website TEXT[],
                timezone VARCHAR(255),
                language VARCHAR(255),
                location VARCHAR(255),
                nums_fans INT DEFAULT 0,
                nums_following INT DEFAULT 0,
                nums_projects INT DEFAULT 0,
                nums_issues INT DEFAULT 0,
                nums_comments INT DEFAULT 0,
                nums_stars INT DEFAULT 0,
                nums_teams INT DEFAULT 0,
                nums_groups INT DEFAULT 0,
                nums_repositories INT DEFAULT 0,
                nums_reviews INT DEFAULT 0,
                allow_use BOOLEAN DEFAULT TRUE,
                allow_create BOOLEAN DEFAULT TRUE,
                max_repository INT DEFAULT 10,
                max_team INT DEFAULT 10,
                max_group INT DEFAULT 10,
                max_project INT DEFAULT 10,
                show_email BOOLEAN DEFAULT TRUE,
                show_active BOOLEAN DEFAULT TRUE,
                show_project BOOLEAN DEFAULT TRUE,
                can_search BOOLEAN DEFAULT TRUE,
                can_follow BOOLEAN DEFAULT TRUE,
                theme VARCHAR(255) DEFAULT 'default',
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                deleted_at TIMESTAMP,
                last_login_at TIMESTAMP
            );
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS security (
                uid UUID PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                description TEXT,
                ip VARCHAR(255),
                user_agent VARCHAR(255),
                device VARCHAR(255),
                location VARCHAR(255),
                action VARCHAR(255) NOT NULL,
                actor VARCHAR(255) NOT NULL,
                actor_uid UUID NOT NULL,
                "user" VARCHAR(255) NOT NULL,
                user_uid UUID NOT NULL,
                timestamp TIMESTAMP NOT NULL
            );
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS access (
                uid UUID PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                description TEXT,
                resource_owner VARCHAR(255) NOT NULL,
                resource_owner_uid UUID NOT NULL,
                expiration VARCHAR(255) NOT NULL,
                fingerprint VARCHAR(255) NOT NULL,
                repo_access INT NOT NULL,
                email_access INT NOT NULL,
                event_access INT NOT NULL,
                follow_access INT NOT NULL,
                gpg_access INT NOT NULL,
                ssh_access INT NOT NULL,
                webhook_access INT NOT NULL,
                wiki_access INT NOT NULL,
                project_access INT NOT NULL,
                issue_access INT NOT NULL,
                comment_access INT NOT NULL,
                profile_access INT NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL
            );
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS follow (
                uid UUID PRIMARY KEY,
                user_id UUID NOT NULL,
                target_id UUID NOT NULL,
                created_at TIMESTAMP NOT NULL
            );
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS ssh (
                uid UUID PRIMARY KEY,
                user_id UUID NOT NULL,
                name VARCHAR(255) NOT NULL,
                fingerprint VARCHAR(255) NOT NULL,
                description TEXT,
                content TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL
            );
            "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS tokens (
                uid UUID PRIMARY KEY,
                user_id UUID NOT NULL,
                name VARCHAR(255) NOT NULL,
                fingerprint VARCHAR(255) NOT NULL,
                description TEXT,
                token TEXT NOT NULL,
                access VARCHAR(255) NOT NULL,
                use_history TEXT[],
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                expires_at TIMESTAMP NOT NULL
            );
            "#,
        )
        .await?;
        Ok(())
    }
}
