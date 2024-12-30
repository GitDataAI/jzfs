use crate::models::groups::groups;
use crate::models::repos::{branchs, labels, license};
use crate::models::teams::teamrepo;
use crate::models::users::{star, users};
use async_graphql::SimpleObject;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject, Serialize, Deserialize)]
#[sea_orm(table_name = "repos")]
#[graphql(name = "repos")]
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
