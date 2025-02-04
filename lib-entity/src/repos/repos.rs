use crate::groups::groups;
use crate::repos::{branchs, labels, license};
use crate::teams::teamrepo;
use crate::users::{star, users};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "repos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub owner: String,
    pub owner_id: Uuid,
    pub avatar_url: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub private: bool,
    pub is_group: bool,
    pub has_issues: bool,
    pub has_idcard: bool,
    pub has_wiki: bool,
    pub has_downloads: bool,
    pub has_projects: bool,
    pub topic: Vec<String>,
    pub collaborators: Vec<Uuid>,
    pub git_http_url: String,
    pub git_ssh_url: String,
    pub default_branchs: Option<String>,
    pub nums_star: i64,
    pub nums_fork: i64,
    pub nums_watcher: i64,
    pub nums_commit: i64,
    pub nums_release: i64,
    pub nums_tag: i64,
    pub nums_branchs: i64,
    pub nums_members: i64,
    pub fork: bool,
    pub fork_from: Option<Uuid>,
    pub created: i64,
    pub updated: i64,
}
impl ActiveModelBehavior for ActiveModel {}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RepoLabels,
    StarModel,
    BranchsModel,
    LicenseModel,
    GroupsModel,
    UserModel,
    TeamRepo,
}
impl Related<labels::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RepoLabels.def()
    }
}
impl Related<branchs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BranchsModel.def()
    }
}
impl Related<star::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StarModel.def()
    }
}
impl Related<license::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LicenseModel.def()
    }
}
impl Related<groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroupsModel.def()
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
            Relation::RepoLabels => Entity::has_many(labels::Entity).into(),
            Relation::StarModel => Entity::has_many(star::Entity).into(),
            Relation::BranchsModel => Entity::has_many(branchs::Entity).into(),
            Relation::LicenseModel => Entity::has_many(license::Entity).into(),
            Relation::GroupsModel => Entity::belongs_to(groups::Entity)
                .from(Column::OwnerId)
                .to(groups::Column::Uid)
                .into(),
            Relation::UserModel => Entity::belongs_to(users::Entity)
                .from(Column::OwnerId)
                .to(users::Column::Uid)
                .into(),
            Relation::TeamRepo => Entity::has_one(teamrepo::Entity).into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RepoCreateOptions {
    pub owner: String,
    pub owner_id: Uuid,
    pub is_group: bool,
    pub private: bool,
    pub name: String,
    pub description: Option<String>,
    pub add_readme: bool,
}


#[derive(DeriveIden)]
pub enum ReposMigration {
    #[sea_orm(iden = "repos")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "owner")]
    Owner,
    #[sea_orm(iden = "owner_id")]
    OwnerId,
    #[sea_orm(iden = "avatar_url")]
    AvatarUrl,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "website")]
    Website,
    #[sea_orm(iden = "private")]
    Private,
    #[sea_orm(iden = "is_group")]
    IsGroup,
    #[sea_orm(iden = "has_issues")]
    HasIssues,
    #[sea_orm(iden = "has_idcard")]
    HasIdcard,
    #[sea_orm(iden = "has_wiki")]
    HasWiki,
    #[sea_orm(iden = "has_downloads")]
    HasDownloads,
    #[sea_orm(iden = "has_projects")]
    HasProjects,
    #[sea_orm(iden = "topic")]
    Topic,
    #[sea_orm(iden = "collaborators")]
    Collaborators,
    #[sea_orm(iden = "git_http_url")]
    GitHttpUrl,
    #[sea_orm(iden = "git_ssh_url")]
    GitSshUrl,
    #[sea_orm(iden = "default_branchs")]
    DefaultBranchs,
    #[sea_orm(iden = "nums_star")]
    NumsStar,
    #[sea_orm(iden = "nums_fork")]
    NumsFork,
    #[sea_orm(iden = "nums_watcher")]
    NumsWatcher,
    #[sea_orm(iden = "nums_commit")]
    NumsCommit,
    #[sea_orm(iden = "nums_release")]
    NumsRelease,#[sea_orm(iden = "nums_tag")]
    NumsTag,
    #[sea_orm(iden = "nums_branchs")]
    NumsBranchs,
    #[sea_orm(iden = "nums_members")]
    NumsMembers,
    #[sea_orm(iden = "fork")]
    Fork,
    #[sea_orm(iden = "fork_from")]
    ForkFrom,
    #[sea_orm(iden = "created")]
    Created,
    #[sea_orm(iden = "updated")]
    Updated,
}

impl ReposMigration {
    pub fn create() -> sea_orm_migration::prelude::TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .table(ReposMigration::Table)
            .if_not_exists()
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::Owner)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::OwnerId)
                    .uuid()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::AvatarUrl)
                    .string()
                    .null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::Name)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::Description)
                    .string()
                    .null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::Website)
                    .string()
                    .null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::Private)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::IsGroup)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::HasIssues)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::HasIdcard)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::HasWiki)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::HasDownloads)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::HasProjects)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::Topic)
                    .array(
                        ColumnType::Text
                    )
                    .null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::Collaborators)
                    .array(
                        ColumnType::Uuid
                    )
                    .null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::GitHttpUrl)
                    .string()
                    .null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::GitSshUrl)
                    .string()
                    .null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::DefaultBranchs)
                    .string()
                    .null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::NumsStar)
                    .integer()
                    .big_integer(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::NumsFork)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::NumsWatcher)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::NumsCommit)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::NumsRelease)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::NumsTag)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::NumsBranchs)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::NumsMembers)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::Fork)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::ForkFrom)
                    .uuid()
                    .null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::Created)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(ReposMigration::Updated)
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