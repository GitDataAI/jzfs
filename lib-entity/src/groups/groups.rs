use sea_orm::*;
use uuid::Uuid;
use crate::repos::repos;
use crate::teams::teams;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "groups")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub bio: Option<String>,
    pub pronouns: String,
    pub company: Option<String>,
    pub location: Option<String>,
    pub localtime: Option<String>,
    pub i18n: Option<String>,
    pub website: Vec<String>,
    pub orcid: Option<String>,
    pub social: Vec<String>,
    pub theme: String,
    pub pinned: Vec<Uuid>,
    pub repository: i64,
    pub package: i64,
    pub release: i64,
    pub mentioned: bool,
    pub contact: String,
    pub visible_email: bool,
    pub pro: bool,
    pub avatar_url: Option<String>,
    pub created: i64,
    pub updated: i64,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    TeamModels,
    RepoModels,
}

impl Related<teams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamModels.def()
    }
}

impl Related<repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RepoModels.def()
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::TeamModels => Entity::has_many(teams::Entity).into(),
            Relation::RepoModels => Entity::has_many(repos::Entity).into(),
        }
    }
}


#[derive(DeriveIden)]
pub enum GroupsMigration {
    #[sea_orm(iden = "groups")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "bio")]
    Bio,
    #[sea_orm(iden = "pronouns")]
    Pronouns,
    #[sea_orm(iden = "company")]
    Company,
    #[sea_orm(iden = "location")]
    Location,
    #[sea_orm(iden = "localtime")]
    Localtime,
    #[sea_orm(iden = "i18n")]
    I18n,
    #[sea_orm(iden = "website")]
    Website,
    #[sea_orm(iden = "orcid")]
    Orcid,
    #[sea_orm(iden = "social")]
    Social,
    #[sea_orm(iden = "theme")]
    Theme,
    #[sea_orm(iden = "pinned")]
    Pinned,
    #[sea_orm(iden = "repository")]
    Repository,
    #[sea_orm(iden = "package")]
    Package,
    #[sea_orm(iden = "release")]
    Release,
    #[sea_orm(iden = "mentioned")]
    Mentioned,
    #[sea_orm(iden = "contact")]
    Contact,
    #[sea_orm(iden = "visible_email")]
    VisibleEmail,
    #[sea_orm(iden = "pro")]
    Pro,
    #[sea_orm(iden = "avatar_url")]
    AvatarUrl,
    #[sea_orm(iden = "created")]
    Created,
    #[sea_orm(iden = "updated")]
    Updated,
}

impl GroupsMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(GroupsMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Name)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Bio)
                    .string(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Pronouns)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Company)
                    .string(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Location)
                    .string(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Localtime)
                    .string(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::I18n)
                    .string(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Website)
                    .array(sea_orm_migration::prelude::ColumnType::Text),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Orcid)
                    .string(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Social)
                    .array(sea_orm_migration::prelude::ColumnType::Text),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Theme)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Pinned)
                    .array(sea_orm_migration::prelude::ColumnType::Uuid),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Repository)
                    .integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Package)
                    .integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Release)
                    .integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Mentioned)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Contact)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::VisibleEmail)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Pro)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::AvatarUrl)
                    .string(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Created)
                    .integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(GroupsMigration::Updated)
                    .integer()
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

