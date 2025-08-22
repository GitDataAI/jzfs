use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS vector;")
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RepoFeatures::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RepoFeatures::RepoUid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RepoFeatures::Vector)
                            .vector(Some(128))
                            .not_null(),
                    )
                    .col(ColumnDef::new(RepoFeatures::Meta).json().not_null())
                    .col(
                        ColumnDef::new(RepoFeatures::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        manager.get_connection()
            .execute_unprepared("CREATE TYPE interaction_type AS ENUM ('clone', 'star', 'fork', 'commit', 'pr', 'view')")
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserInteractions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserInteractions::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(UserInteractions::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserInteractions::RepoId).uuid().not_null())
                    .col(
                        ColumnDef::new(UserInteractions::Act)
                            .custom("interaction_type")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserInteractions::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserInteractions::Weight)
                            .float()
                            .not_null()
                            .default(1.0),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. 创建或更新user_preferences表
        manager
            .create_table(
                Table::create()
                    .table(UserPreferences::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserPreferences::UserId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserPreferences::Topics)
                            .array(ColumnType::Text)
                            .not_null()
                            .default("{}"),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. 创建或更新cf_scores表
        manager
            .create_table(
                Table::create()
                    .table(CfScores::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CfScores::CfUid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(CfScores::UserId).uuid().not_null())
                    .col(ColumnDef::new(CfScores::RepoId).uuid().not_null())
                    .col(ColumnDef::new(CfScores::Score).float().not_null())
                    .col(
                        ColumnDef::new(CfScores::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(CfScores::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 5. 创建或更新hybrid_recommendations表
        manager
            .create_table(
                Table::create()
                    .table(HybridRecommendations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(HybridRecommendations::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(
                        ColumnDef::new(HybridRecommendations::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HybridRecommendations::RepoId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(HybridRecommendations::Rank)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(HybridRecommendations::Reason).text().null())
                    .col(
                        ColumnDef::new(HybridRecommendations::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 添加索引以提高查询性能
        manager
            .create_index(
                Index::create()
                    .name("idx_user_interactions_user_id")
                    .table(UserInteractions::Table)
                    .col(UserInteractions::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_interactions_repo_id")
                    .table(UserInteractions::Table)
                    .col(UserInteractions::RepoId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cf_scores_user_id")
                    .table(CfScores::Table)
                    .col(CfScores::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cf_scores_repo_id")
                    .table(CfScores::Table)
                    .col(CfScores::RepoId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_hybrid_recommendations_user_id")
                    .table(HybridRecommendations::Table)
                    .col(HybridRecommendations::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除索引
        manager
            .drop_index(
                Index::drop()
                    .name("idx_hybrid_recommendations_user_id")
                    .table(HybridRecommendations::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_cf_scores_repo_id")
                    .table(CfScores::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_cf_scores_user_id")
                    .table(CfScores::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_interactions_repo_id")
                    .table(UserInteractions::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_interactions_user_id")
                    .table(UserInteractions::Table)
                    .to_owned(),
            )
            .await?;

        // 删除表
        manager
            .drop_table(Table::drop().table(HybridRecommendations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CfScores::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserPreferences::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserInteractions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RepoFeatures::Table).to_owned())
            .await?;

        // 删除枚举类型
        manager
            .get_connection()
            .execute_unprepared("DROP TYPE IF EXISTS interaction_type")
            .await?;

        Ok(())
    }
}

// 定义表名和列名
#[derive(Iden)]
pub enum RepoFeatures {
    Table,
    RepoUid,
    Vector,
    Meta,
    UpdatedAt,
}

#[derive(Iden)]
pub enum UserInteractions {
    Table,
    Uid,
    UserId,
    RepoId,
    Act,
    CreatedAt,
    Weight,
}

#[derive(Iden)]
pub enum UserPreferences {
    Table,
    UserId,
    Topics,
}

#[derive(Iden)]
pub enum CfScores {
    Table,
    CfUid,
    UserId,
    RepoId,
    Score,
    UpdatedAt,
    CreatedAt,
}

#[derive(Iden)]
pub enum HybridRecommendations {
    Table,
    Uid,
    UserId,
    RepoId,
    Rank,
    Reason,
    CreatedAt,
}
