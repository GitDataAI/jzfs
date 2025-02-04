use sea_orm::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "emails")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub main: bool,
    pub primary: bool,
    pub created: i64,
    pub updated: i64,
    pub hasused: i64,
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
pub enum EmailMigration {
    #[sea_orm(iden = "emails")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "content")]
    Content,
    #[sea_orm(iden = "main")]
    Main,
    #[sea_orm(iden = "primary")]
    Primary,
    #[sea_orm(iden = "created")]
    Created,
    #[sea_orm(iden = "updated")]
    Updated,
    #[sea_orm(iden = "hasused")]
    Hasused,
}

impl EmailMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(EmailMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(EmailMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(EmailMigration::UserId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(EmailMigration::Content)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(EmailMigration::Main)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(EmailMigration::Primary)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(EmailMigration::Created)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(EmailMigration::Updated)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(EmailMigration::Hasused)
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
