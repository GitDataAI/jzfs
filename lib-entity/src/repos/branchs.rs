use sea_orm::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "branchs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub name: String,
    pub head: Option<String>,
    pub protect: bool,
}
impl ActiveModelBehavior for ActiveModel {}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RepoModel,
    Commits,
}
impl Related<super::repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RepoModel.def()
    }
}
impl Related<super::commits::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Commits.def()
    }
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::RepoModel => Entity::belongs_to(super::repos::Entity)
                .from(Column::RepoId)
                .to(super::repos::Column::Uid)
                .into(),
            Relation::Commits => Entity::has_many(super::commits::Entity).into(),
        }
    }
}

#[derive(DeriveIden)]
pub enum BranchMigration {
    #[sea_orm(iden = "branchs")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "repo_id")]
    RepoId,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "head")]
    Head,
    #[sea_orm(iden = "protect")]
    Protect,
}
impl BranchMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(BranchMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(BranchMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(BranchMigration::RepoId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(BranchMigration::Name)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(BranchMigration::Head)
                    .string(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(BranchMigration::Protect)
                    .boolean()
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