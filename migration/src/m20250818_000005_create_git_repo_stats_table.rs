use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GitRepoStats::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GitRepoStats::Uid)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GitRepoStats::RepoUid).uuid().not_null())
                    .col(
                        ColumnDef::new(GitRepoStats::Stars)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(GitRepoStats::Watches)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(GitRepoStats::Forks)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(GitRepoStats::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(GitRepoStats::UpdatedAt)
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
            .drop_table(Table::drop().table(GitRepoStats::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum GitRepoStats {
    Table,
    Uid,
    RepoUid,
    Stars,
    Watches,
    Forks,
    CreatedAt,
    UpdatedAt,
}
