use sea_orm::*;
use uuid::Uuid;
use crate::groups::groups;
use crate::users::users;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "teaminv")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub team_id: Uuid,
    pub token: String,
    pub origin: Uuid,
    pub expire: i64,
    pub access: i64,
    pub created: i64,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    TeamModel,
    UserModel,
    GroupModel,
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
                .from(Column::Origin)
                .to(users::Column::Uid)
                .into(),
            Relation::GroupModel => Entity::belongs_to(groups::Entity)
                .from(Column::Origin)
                .to(groups::Column::Uid)
                .into(),
        }
    }
}

#[derive(DeriveIden)]
pub enum TeamInvMigration {
    #[sea_orm(iden = "teaminv")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "team_id")]
    TeamId,
    #[sea_orm(iden = "token")]
    Token,
    #[sea_orm(iden = "origin")]
    Origin,
    #[sea_orm(iden = "expire")]
    Expire,
    #[sea_orm(iden = "access")]
    Access,
    #[sea_orm(iden = "created")]
    Created,
}

impl TeamInvMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(TeamInvMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamInvMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamInvMigration::TeamId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamInvMigration::Token)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamInvMigration::Origin)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamInvMigration::Expire)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamInvMigration::Access)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamInvMigration::Created)
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
