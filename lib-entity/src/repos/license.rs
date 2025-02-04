use sea_orm::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "license")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub name: String,
    pub license: String,
    pub created: i64,
}
impl ActiveModelBehavior for ActiveModel {}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RepoModel,
}
impl Related<super::repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RepoModel.def()
    }
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::RepoModel => Entity::belongs_to(super::repos::Entity)
                .from(Column::RepoId)
                .to(super::repos::Column::Uid)
                .into(),
        }
    }
}

#[derive(DeriveIden)]
pub enum LicenseMigration {
    #[sea_orm(iden = "license")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "repo_id")]
    RepoId,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "license")]
    License,
    #[sea_orm(iden = "created")]
    Created,
}

impl LicenseMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(LicenseMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(LicenseMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(LicenseMigration::RepoId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(LicenseMigration::Name)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(LicenseMigration::License)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(LicenseMigration::Created)
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