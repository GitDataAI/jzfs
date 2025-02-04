use uuid::Uuid;

use sea_orm::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "followers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub follower_id: Uuid,
    pub created: i64,
}
impl ActiveModelBehavior for ActiveModel {}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserModel,
    FollowerModel,
}
impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserModel.def()
    }
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::UserModel => Entity::belongs_to(super::users::Entity)
                .from(Column::UserId)
                .to(super::users::Column::Uid)
                .into(),
            Relation::FollowerModel => Entity::belongs_to(super::users::Entity)
                .from(Column::FollowerId)
                .to(super::users::Column::Uid)
                .into(),
        }
    }
}

#[derive(DeriveIden)]
pub enum FollowerMigration {
    #[sea_orm(iden = "followers")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "follower_id")]
    FollowerId,
    #[sea_orm(iden = "created")]
    Created,
}

impl FollowerMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(FollowerMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(FollowerMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(FollowerMigration::UserId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(FollowerMigration::FollowerId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(FollowerMigration::Created)
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