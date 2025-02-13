use sea_orm::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "commits")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid : Uuid,
    pub repo_id : Uuid,
    pub branch_id : Uuid,
    pub description : String,
    pub commit_user : String,
    pub commit_email : String,
    pub commit_id : String,
    pub status : i32,
    pub created : i64,
}
impl ActiveModelBehavior for ActiveModel {}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RepoModel,
    BranchModel,
}
impl Related<super::repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RepoModel.def()
    }
}
impl Related<super::branchs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BranchModel.def()
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::RepoModel => Entity::belongs_to(super::repos::Entity)
                .from(Column::RepoId)
                .to(super::repos::Column::Uid)
                .into(),
            Relation::BranchModel => Entity::belongs_to(super::branchs::Entity)
                .from(Column::BranchId)
                .to(super::branchs::Column::Uid)
                .into(),
        }
    }
}

#[derive(DeriveIden)]
pub enum CommitMigration {
    #[sea_orm(iden = "commits")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "repo_id")]
    RepoId,
    #[sea_orm(iden = "branch_id")]
    BranchId,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "commit_user")]
    CommitUser,
    #[sea_orm(iden = "commit_id")]
    CommitId,
    #[sea_orm(iden = "created")]
    Created,
    #[sea_orm(iden = "commit_email")]
    CommitEmail,
    #[sea_orm(iden = "status")]
    Status,
}

impl CommitMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(CommitMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(CommitMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(CommitMigration::RepoId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(CommitMigration::BranchId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(CommitMigration::Description)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(CommitMigration::CommitUser)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(CommitMigration::CommitEmail)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(CommitMigration::CommitId)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(CommitMigration::Created)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(CommitMigration::Status)
                    .integer()
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
