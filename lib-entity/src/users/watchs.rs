use crate::repos::repos;
use sea_orm::*;
use sea_orm_migration::prelude::TableCreateStatement;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "watchs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub repo_id: Uuid,
    pub level: i32,
    pub created_at: i64,
}
impl ActiveModelBehavior for ActiveModel {}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Users,
    Repos,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}
impl Related<repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Repos.def()
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users => Entity::belongs_to(super::users::Entity)
                .from(Column::UserId)
                .to(super::users::Column::Uid)
                .into(),
            Self::Repos => Entity::belongs_to(repos::Entity)
                .from(Column::RepoId)
                .to(repos::Column::Uid)
                .into(),
        }
    }
}

#[derive(DeriveIden)]
pub enum WatchUserMigration {
    #[sea_orm(iden = "watchs")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "repo_id")]
    RepoId,
    #[sea_orm(iden = "level")]
    Level,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}

impl WatchUserMigration {
    pub fn create() -> TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(WatchUserMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(WatchUserMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(WatchUserMigration::UserId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(WatchUserMigration::RepoId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(WatchUserMigration::Level)
                    .integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(WatchUserMigration::CreatedAt)
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