use uuid::Uuid;

use sea_orm::*;
use crate::users::users;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "teamus")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub join_at: i64,
    pub access: i64,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    TeamModel,
    UserModel,
}

impl Related<super::teams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamModel.def()
    }
}

impl Related<users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserModel.def()
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::TeamModel => Entity::belongs_to(super::teams::Entity)
                .from(Column::TeamId)
                .to(super::teams::Column::Uid)
                .into(),
            Relation::UserModel => Entity::belongs_to(users::Entity)
                .from(Column::UserId)
                .to(users::Column::Uid)
                .into(),
        }
    }
}


#[derive(DeriveIden)]
pub enum TeamUsMigration {
    #[sea_orm(iden = "teamus")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "team_id")]
    TeamId,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "join_at")]
    JoinAt,
    #[sea_orm(iden = "access")]
    Access,
}

impl TeamUsMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(TeamUsMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamUsMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamUsMigration::TeamId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamUsMigration::UserId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamUsMigration::JoinAt)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamUsMigration::Access)
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