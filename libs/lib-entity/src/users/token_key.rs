use sea_orm::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "token_keys")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid : Uuid,
    pub user_id : Uuid,
    #[serde(skip_serializing)]
    pub content : String,
    pub name : String,
    pub access : i64,
    pub created : i64,
    pub updated : i64,
    pub expire : i64,
    pub hasused : i64,
}
impl ActiveModelBehavior for ActiveModel {}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserModel,
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
        }
    }
}

#[derive(DeriveIden)]
pub enum TokenKeyMigration {
    #[sea_orm(iden = "token_keys")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "content")]
    Content,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "access")]
    Access,
    #[sea_orm(iden = "created")]
    Created,
    #[sea_orm(iden = "updated")]
    Updated,
    #[sea_orm(iden = "expire")]
    Expire,
    #[sea_orm(iden = "hasused")]
    Hasused,
}

impl TokenKeyMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(TokenKeyMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TokenKeyMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TokenKeyMigration::UserId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TokenKeyMigration::Content)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TokenKeyMigration::Name)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TokenKeyMigration::Access)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TokenKeyMigration::Created)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TokenKeyMigration::Updated)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TokenKeyMigration::Expire)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TokenKeyMigration::Hasused)
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
