use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 向user_repo_active表添加name字段
        manager
            .alter_table(
                Table::alter()
                    .table(UserRepoActive::Table)
                    .add_column(
                        ColumnDef::new(UserRepoActive::Name)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        // 向user_repo_active表添加email字段
        manager
            .alter_table(
                Table::alter()
                    .table(UserRepoActive::Table)
                    .add_column(
                        ColumnDef::new(UserRepoActive::Email)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 从user_repo_active表移除email字段
        manager
            .alter_table(
                Table::alter()
                    .table(UserRepoActive::Table)
                    .drop_column(UserRepoActive::Email)
                    .to_owned(),
            )
            .await?;

        // 从user_repo_active表移除name字段
        manager
            .alter_table(
                Table::alter()
                    .table(UserRepoActive::Table)
                    .drop_column(UserRepoActive::Name)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum UserRepoActive {
    Table,
    Name,
    Email,
}
