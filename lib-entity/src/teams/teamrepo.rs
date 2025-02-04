use sea_orm::*;
use uuid::Uuid;
use crate::repos::repos;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "teamrepo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub team_id: Uuid,
    pub access: i64,
    pub created: i64,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RepoModel,
    TeamModel,
}

impl Related<repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RepoModel.def()
    }
}

impl Related<super::teams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamModel.def()
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::RepoModel => Entity::belongs_to(repos::Entity)
                .from(Column::RepoId)
                .to(repos::Column::Uid)
                .into(),
            Relation::TeamModel => Entity::belongs_to(super::teams::Entity)
                .from(Column::TeamId)
                .to(super::teams::Column::Uid)
                .into(),
        }
    }
}

#[derive(DeriveIden)]
pub enum TeamRepoMigration {
    #[sea_orm(iden = "teamrepo")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "repo_id")]
    RepoId,
    #[sea_orm(iden = "team_id")]
    TeamId,
    #[sea_orm(iden = "access")]
    Access,
    #[sea_orm(iden = "created")]
    Created,
}

impl TeamRepoMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(TeamRepoMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamRepoMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamRepoMigration::RepoId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamRepoMigration::TeamId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamRepoMigration::Access)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(TeamRepoMigration::Created)
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
