use sea_orm::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "labels")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub url: String,
    pub name: String,
    pub color: String,
    #[sea_orm(column_name = "description")]
    pub description: Option<String>,
    pub created: i64,
}
impl ActiveModelBehavior for ActiveModel {}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    ModelHasRepoLabels,
}
impl Related<super::repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelHasRepoLabels.def()
    }
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::ModelHasRepoLabels => Entity::belongs_to(super::repos::Entity)
                .from(Column::RepoId)
                .to(super::repos::Column::Uid)
                .into(),
        }
    }
}

#[derive(DeriveIden)]
pub enum LabelsMigration {
    #[sea_orm(iden = "labels")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "repo_id")]
    RepoId,
    #[sea_orm(iden = "url")]
    Url,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "color")]
    Color,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "created")]
    Created,
}

impl LabelsMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(LabelsMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(LabelsMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(LabelsMigration::RepoId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(LabelsMigration::Url)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(LabelsMigration::Name)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(LabelsMigration::Color)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(LabelsMigration::Description)
                    .string(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(LabelsMigration::Created)
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