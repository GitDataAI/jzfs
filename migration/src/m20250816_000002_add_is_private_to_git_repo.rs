use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 向git_repo表添加is_private字段
        manager
            .alter_table(
                Table::alter()
                    .table(GitRepo::Table)
                    .add_column(
                        ColumnDef::new(GitRepo::IsPrivate)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 从git_repo表移除is_private字段
        manager
            .alter_table(
                Table::alter()
                    .table(GitRepo::Table)
                    .drop_column(GitRepo::IsPrivate)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

// 表名和列名定义
#[derive(Iden)]
enum GitRepo {
    Table,
    IsPrivate,
}
