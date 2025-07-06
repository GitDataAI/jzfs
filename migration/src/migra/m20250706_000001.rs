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
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS security (
            uid UUID PRIMARY KEY,
            title VARCHAR NOT NULL,
            description TEXT,
            ip VARCHAR,
            user_agent VARCHAR,
            device VARCHAR,
            location VARCHAR,
            action VARCHAR NOT NULL,
            actor VARCHAR NOT NULL,
            actor_uid UUID NOT NULL,
            "user" VARCHAR NOT NULL,
            user_uid UUID NOT NULL,
            timestamp TIMESTAMP NOT NULL DEFAULT now()
        );
        "#,)
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS ssh (
            uid UUID PRIMARY KEY,
            user_id UUID NOT NULL,
            name VARCHAR NOT NULL,
            fingerprint VARCHAR NOT NULL,
            description TEXT,
            content TEXT NOT NULL UNIQUE,
            created_at TIMESTAMP NOT NULL DEFAULT now(),
            updated_at TIMESTAMP NOT NULL DEFAULT now()
        );
        "#,
        )
        .await?;
        db.execute_unprepared(r#"
        CREATE TABLE IF NOT EXISTS users (
        uid UUID PRIMARY KEY,
        username VARCHAR NOT NULL,
        password TEXT NOT NULL,
        email VARCHAR NOT NULL,
    
        description TEXT,
        avatar TEXT,
        website TEXT[],
        timezone TEXT,
        language TEXT,
        location TEXT,
    
        nums_fans INTEGER NOT NULL,
        nums_following INTEGER NOT NULL,
        nums_projects INTEGER NOT NULL,
        nums_issues INTEGER NOT NULL,
        nums_comments INTEGER NOT NULL,
        nums_stars INTEGER NOT NULL,
        nums_teams INTEGER NOT NULL,
        nums_groups INTEGER NOT NULL,
        nums_repositories INTEGER NOT NULL,
        nums_reviews INTEGER NOT NULL,
    
        allow_use BOOLEAN NOT NULL,
        allow_create BOOLEAN NOT NULL,
        max_repository INTEGER NOT NULL,
        max_team INTEGER NOT NULL,
        max_group INTEGER NOT NULL,
        max_project INTEGER NOT NULL,
    
        show_email BOOLEAN NOT NULL,
        show_active BOOLEAN NOT NULL,
        show_project BOOLEAN NOT NULL,
    
        can_search BOOLEAN NOT NULL,
        can_follow BOOLEAN NOT NULL,
    
        theme TEXT NOT NULL,
    
        created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
        updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
        deleted_at TIMESTAMP WITHOUT TIME ZONE,
        last_login_at TIMESTAMP WITHOUT TIME ZONE
    );
        "#)
        .await?;
        Ok(())
        
    }
}
