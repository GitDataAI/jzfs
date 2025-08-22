use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建user_star_repo表的触发器
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE OR REPLACE FUNCTION update_star_count()
                RETURNS TRIGGER AS $$
                BEGIN
                    IF (TG_OP = 'INSERT') THEN
                        INSERT INTO git_repo_stats (uid, repo_uid, stars, watches, forks, created_at, updated_at)
                        VALUES (uuid_generate_v4(), NEW.repo_id, 1, 0, 0, NOW(), NOW())
                        ON CONFLICT (repo_uid) DO UPDATE
                        SET stars = git_repo_stats.stars + 1,
                            updated_at = NOW();
                    ELSIF (TG_OP = 'DELETE') THEN
                        UPDATE git_repo_stats
                        SET stars = stars - 1,
                            updated_at = NOW()
                        WHERE repo_uid = OLD.repo_id;
                    END IF;
                    RETURN NULL;
                END;
                $$ LANGUAGE plpgsql;",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TRIGGER user_star_repo_trigger
                AFTER INSERT OR DELETE ON user_star_repo
                FOR EACH ROW EXECUTE FUNCTION update_star_count();",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE OR REPLACE FUNCTION update_watch_count()
                RETURNS TRIGGER AS $$
                BEGIN
                    IF (TG_OP = 'INSERT') THEN
                        INSERT INTO git_repo_stats (uid, repo_uid, stars, watches, forks, created_at, updated_at)
                        VALUES (uuid_generate_v4(), NEW.repo_id, 0, 1, 0, NOW(), NOW())
                        ON CONFLICT (repo_uid) DO UPDATE
                        SET watches = git_repo_stats.watches + 1,
                            updated_at = NOW();
                    ELSIF (TG_OP = 'DELETE') THEN
                        UPDATE git_repo_stats
                        SET watches = watches - 1,
                            updated_at = NOW()
                        WHERE repo_uid = OLD.repo_id;
                    END IF;
                    RETURN NULL;
                END;
                $$ LANGUAGE plpgsql;",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TRIGGER user_watch_repo_trigger
                AFTER INSERT OR DELETE ON user_watch_repo
                FOR EACH ROW EXECUTE FUNCTION update_watch_count();",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除触发器
        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS user_star_repo_trigger ON user_star_repo;")
            .await?;
        manager
            .get_connection()
            .execute_unprepared(
                "DROP TRIGGER IF EXISTS user_watch_repo_trigger ON user_watch_repo;",
            )
            .await?;

        // 删除触发函数
        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS update_star_count;")
            .await?;
        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS update_watch_count;")
            .await?;

        Ok(())
    }
}
