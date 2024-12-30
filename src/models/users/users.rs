use crate::models::repos::repos;
use crate::models::teams::teamsus;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub localtime: Option<String>,
    pub i18n: Option<String>,
    pub website: Vec<String>,
    pub orcid: Option<String>,
    pub social: Vec<String>,
    pub theme: String,
    pub pinned: Vec<Uuid>,
    pub followers: i32,
    pub following: i32,
    pub repository: i32,
    pub stars: i32,
    pub watching: i32,
    pub package: i32,
    pub release: i32,
    pub mentioned: bool,
    pub main_email: String,
    pub visible_email: bool,
    pub pro: bool,
    #[serde(skip_serializing)]
    pub password: String,
    pub avatar_url: Option<String>,
    pub created: i64,
    pub updated: i64,
    pub hasused: i64,
}
impl ActiveModelBehavior for ActiveModel {}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    EmailsModels,
    SshKeysModels,
    TokenKeysModels,
    StarModels,
    FollowerModel,
    TwoFactor,
    TeamsModel,
    RepoModel,
}
impl Related<super::email::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EmailsModels.def()
    }
}
impl Related<super::ssh_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SshKeysModels.def()
    }
}
impl Related<super::token_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TokenKeysModels.def()
    }
}
impl Related<super::star::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StarModels.def()
    }
}
impl Related<super::follower::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FollowerModel.def()
    }
}
impl Related<super::two_factor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TwoFactor.def()
    }
}
impl Related<teamsus::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamsModel.def()
    }
}
impl Related<repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RepoModel.def()
    }
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::EmailsModels => Entity::has_many(super::email::Entity).into(),
            Relation::SshKeysModels => Entity::has_many(super::ssh_key::Entity).into(),
            Relation::TokenKeysModels => Entity::has_many(super::token_key::Entity).into(),
            Relation::StarModels => Entity::has_many(super::star::Entity).into(),
            Relation::FollowerModel => Entity::has_many(super::follower::Entity).into(),
            Relation::TwoFactor => Entity::has_many(super::two_factor::Entity).into(),
            Relation::TeamsModel => Entity::has_many(teamsus::Entity).into(),
            Relation::RepoModel => Entity::has_many(repos::Entity).into(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateOption {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub localtime: Option<String>,
    pub i18n: Option<String>,
    pub website: Option<Vec<String>>,
    pub orcid: Option<String>,
    pub social: Option<Vec<String>>,
    pub theme: Option<String>,
    pub pinned: Option<Vec<Uuid>>,
}
