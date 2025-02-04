use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::groups::groups;
use crate::teams::teamrepo;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created: i64,
    pub updated: i64,
    pub created_by: Uuid,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    OrganizationModel,
    TeamRepo,
}

impl Related<groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrganizationModel.def()
    }
}
impl Related<teamrepo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamRepo.def()
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::OrganizationModel => Entity::belongs_to(groups::Entity)
                .from(Column::OrgId)
                .to(groups::Column::Uid)
                .into(),
            Relation::TeamRepo => Entity::has_many(teamrepo::Entity).into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamCreateOption {
    pub org_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}


#[derive(DeriveIden)]
pub enum TeamsMigration {
    #[sea_orm(iden = "teams")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "org_id")]
    OrgId,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "created")]
    Created,
    #[sea_orm(iden = "updated")]
    Updated,
    #[sea_orm(iden = "created_by")]
    CreatedBy,
}

impl TeamsMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(TeamsMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamsMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamsMigration::OrgId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamsMigration::Name)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamsMigration::Description)
                    .string(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamsMigration::Created)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamsMigration::Updated)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamsMigration::CreatedBy)
                    .uuid()
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
