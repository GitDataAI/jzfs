use sea_orm::*;
use serde::Serialize;
use uuid::Uuid;
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "stars")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid : Uuid,
    pub user_id : Uuid,
    pub repo_id : Uuid,
    pub created : i64,
}
impl ActiveModelBehavior for ActiveModel {}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserModel,
    RepoModel,
}
impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserModel.def()
    }
}
impl Related<super::super::repos::repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RepoModel.def()
    }
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::UserModel => Entity::belongs_to(super::users::Entity)
                .from(Column::UserId)
                .to(super::users::Column::Uid)
                .into(),
            Relation::RepoModel => Entity::belongs_to(super::super::repos::repos::Entity)
                .from(Column::RepoId)
                .to(super::super::repos::repos::Column::Uid)
                .into(),
        }
    }
}

#[derive(DeriveIden)]
pub enum StarMigration {
    #[sea_orm(iden = "stars")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "repo_id")]
    RepoId,
    #[sea_orm(iden = "created")]
    Created,
}

impl StarMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(StarMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(StarMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(StarMigration::UserId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(StarMigration::RepoId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(StarMigration::Created)
                    .big_integer()
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
