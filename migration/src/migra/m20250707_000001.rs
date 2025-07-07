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
        "#,
        )
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
        db.execute_unprepared(
            r#"
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
        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS git_branch (
                                          uid UUID PRIMARY KEY,
                                          repo_uid UUID NOT NULL,
                                          protect BOOLEAN NOT NULL,
                                          name TEXT NOT NULL,
                                          head TEXT NOT NULL,
                                          time TEXT NOT NULL
);
        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
CREATE TABLE IF NOT EXISTS git_code (
                                        uid UUID PRIMARY KEY,
                                        repo_uid UUID NOT NULL,
                                        language JSONB NOT NULL
);"#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS watch (
                                     uid UUID PRIMARY KEY,
                                     user_id UUID NOT NULL,
                                     repo_uid UUID NOT NULL,
                                     level INTEGER NOT NULL,
                                     created_at TIMESTAMPTZ NOT NULL
);
        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS git_data (
                                        uid UUID PRIMARY KEY,
                                        repo_uid UUID NOT NULL,
                                        task JSONB NOT NULL,
                                        modalities JSONB NOT NULL,
                                        format TEXT NOT NULL,
                                        language TEXT,
                                        tage JSONB NOT NULL
);
        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS org_team (
                                        uid UUID PRIMARY KEY,
                                        org_uid UUID NOT NULL,
                                        team_uid UUID NOT NULL,
                                        created_at TIMESTAMPTZ NOT NULL
);

        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS git_commit (
                                          uid UUID PRIMARY KEY,
                                          sha TEXT NOT NULL,
                                          branch_uid UUID NOT NULL,
                                          repo_uid UUID NOT NULL,
                                          branch_name TEXT NOT NULL,
                                          message TEXT NOT NULL,
                                          author_name TEXT NOT NULL,
                                          author_email TEXT NOT NULL,
                                          commiter_name TEXT NOT NULL,
                                          commiter_email TEXT NOT NULL,
                                          status INTEGER NOT NULL,
                                          runner JSONB NOT NULL,
                                          time TEXT NOT NULL,
                                          created_at TIMESTAMPTZ NOT NULL
);

        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS repository (
                                          uid UUID PRIMARY KEY,
                                          name TEXT NOT NULL,
                                          description TEXT,
                                          website TEXT,
                                          project JSONB NOT NULL,
                                          is_private BOOLEAN NOT NULL,
                                          fork UUID,
                                          nums_fork INTEGER NOT NULL,
                                          nums_star INTEGER NOT NULL,
                                          nums_watch INTEGER NOT NULL,
                                          nums_issue INTEGER NOT NULL,
                                          nums_release INTEGER NOT NULL,
                                          topic JSONB NOT NULL,
                                          status TEXT NOT NULL,
                                          rtype TEXT NOT NULL,
                                          storage UUID NOT NULL,
                                          license TEXT,
                                          created_at TIMESTAMPTZ NOT NULL,
                                          updated_at TIMESTAMPTZ NOT NULL,
                                          created_by UUID NOT NULL
);
        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS blacklist (
                                         uid UUID PRIMARY KEY,
                                         user_id UUID NOT NULL,
                                         target_id UUID NOT NULL,
                                         description TEXT,
                                         created_at TIMESTAMPTZ NOT NULL
);
        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS git_model (
                                         uid UUID PRIMARY KEY,
                                         repo_uid UUID NOT NULL,
                                         size TEXT NOT NULL,
                                         tensor TEXT NOT NULL,
                                         category JSONB NOT NULL,
                                         multimodal TEXT,
                                         language TEXT,
                                         paper TEXT
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
                                     created_at TIMESTAMPTZ NOT NULL
);
        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS org_repo (
                                        uid UUID PRIMARY KEY,
                                        org_uid UUID NOT NULL,
                                        repo_uid UUID NOT NULL,
                                        created_at TIMESTAMPTZ NOT NULL
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
                                      special BOOLEAN NOT NULL,
                                      created_at TIMESTAMPTZ NOT NULL
);

        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS git_repo_nums (
                                             uid UUID PRIMARY KEY,
                                             repo_uid UUID NOT NULL,
                                             nums_pullrequest INTEGER NOT NULL,
                                             nums_commit INTEGER NOT NULL,
                                             nums_release INTEGER NOT NULL,
                                             nums_tag INTEGER NOT NULL,
                                             nums_branch INTEGER NOT NULL
);
        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS user_repo (
                                         uid UUID PRIMARY KEY,
                                         org_uid UUID NOT NULL,
                                         repo_uid UUID NOT NULL,
                                         created_at TIMESTAMPTZ NOT NULL
);
        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS contributors (
                                            uid UUID PRIMARY KEY,
                                            id SERIAL NOT NULL,
                                            user_id UUID,
                                            repo_id UUID NOT NULL,
                                            email TEXT NOT NULL,
                                            name TEXT NOT NULL
);

        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS issue_history (
                                             issue_id INTEGER NOT NULL,
                                             history_id INTEGER NOT NULL,
                                             action TEXT NOT NULL,
                                             old_value TEXT,
                                             new_value TEXT,
                                             changed_by INTEGER NOT NULL,
                                             created_at TIMESTAMPTZ NOT NULL,
                                             PRIMARY KEY (issue_id, history_id)
);
        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
     CREATE TABLE IF NOT EXISTS label (
                                     label_uid UUID PRIMARY KEY,
                                     issue_uid UUID NOT NULL,
                                     name TEXT NOT NULL,
                                     description TEXT,
                                     color TEXT NOT NULL,
                                     created_at TIMESTAMPTZ NOT NULL
);
   
        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS issues (
                                      uid UUID PRIMARY KEY,
                                      issue_id SERIAL NOT NULL,
                                      repo_uid UUID NOT NULL,
                                      title TEXT NOT NULL,
                                      description TEXT,
                                      author_uid UUID NOT NULL,
                                      assignee_uid UUID,
                                      state TEXT NOT NULL,
                                      priority_label_uid UUID,
                                      created_at TIMESTAMPTZ NOT NULL,
                                      updated_at TIMESTAMPTZ NOT NULL,
                                      is_deleted BOOLEAN NOT NULL
);
        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS issue_labels (
                                            uid UUID PRIMARY KEY,
                                            issue_uid UUID NOT NULL,
                                            issue_label_uid UUID NOT NULL,
                                            name TEXT NOT NULL,
                                            description TEXT,
                                            color TEXT NOT NULL,
                                            created_at TIMESTAMPTZ NOT NULL
);

        "#,
        )
        .await?;
        db.execute_unprepared(
            r#"
        CREATE TABLE IF NOT EXISTS comment (
                                       uuid UUID PRIMARY KEY,
                                       issue_uid UUID NOT NULL,
                                       comment_uid UUID NOT NULL,
                                       content TEXT NOT NULL,
                                       author_uid UUID NOT NULL,
                                       parent_comment_uid UUID,
                                       created_at TIMESTAMPTZ NOT NULL,
                                       is_deleted BOOLEAN NOT NULL
);
        "#,
        )
        .await?;
        db.execute_unprepared(r#"
        CREATE TABLE IF NOT EXISTS issue_sub(
             uuid UUID PRIMARY KEY,
             user_uid UUID NOT NULL,
             issue_uid UUID NOT NULL,
             created_at TIMESTAMPTZ NOT NULL
        );
        "#,
        )
        .await?;
        Ok(())
    }
}
