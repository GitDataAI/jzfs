use sea_orm::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "blobtree")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid : Uuid,
    pub repo_id : Uuid,
    pub commit_id : Uuid,
    pub branch : String,
    pub tree : JsonValue,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::repos::Entity",
        from = "Column::RepoId",
        to = "super::repos::Column::Uid",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Repos,
}

impl Related<super::repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Repos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(DeriveIden)]
pub enum BlobTreeMigration {
    #[sea_orm(iden = "blobtree")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "repo_id")]
    RepoId,
    #[sea_orm(iden = "commit_id")]
    CommitId,
    #[sea_orm(iden = "branch")]
    Branch,
    #[sea_orm(iden = "tree")]
    Tree,
}

impl BlobTreeMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(BlobTreeMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(BlobTreeMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(BlobTreeMigration::RepoId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(BlobTreeMigration::CommitId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(BlobTreeMigration::Branch)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(BlobTreeMigration::Tree)
                    .json()
                    .not_null(),
            )
            .take()
    }
    pub fn drop() -> sea_orm_migration::prelude::TableDropStatement {
        sea_orm_migration::prelude::Table::drop()
            .table(Self::Table)
            .if_exists()
            .take()
    }
}
