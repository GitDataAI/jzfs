use sea_orm::*;
use sea_orm_migration::prelude::TableCreateStatement;
use uuid::Uuid;
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "two_factor")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub secret: String,
    pub created: i64,
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
pub enum TwoFactorMigration {
    #[sea_orm(iden = "two_factor")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "secret")]
    Secret,
    #[sea_orm(iden = "created")]
    Created,
}
impl TwoFactorMigration {
    pub fn create() -> TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(TwoFactorMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TwoFactorMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TwoFactorMigration::UserId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TwoFactorMigration::Secret)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TwoFactorMigration::Created)
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