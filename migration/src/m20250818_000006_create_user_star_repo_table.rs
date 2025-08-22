use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserStarRepo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserStarRepo::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserStarRepo::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserStarRepo::RepoId).uuid().not_null())
                    .col(
                        ColumnDef::new(UserStarRepo::CreatedAt)
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
            .drop_table(Table::drop().table(UserStarRepo::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum UserStarRepo {
    Table,
    Uid,
    UserId,
    RepoId,
    CreatedAt,
}
