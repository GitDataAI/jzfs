use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserWatchRepo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserWatchRepo::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserWatchRepo::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserWatchRepo::RepoId).uuid().not_null())
                    .col(
                        ColumnDef::new(UserWatchRepo::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserWatchRepo::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum UserWatchRepo {
    Table,
    Uid,
    UserId,
    RepoId,
    CreatedAt,
}
