use crate::models::repos::repos;
use crate::models::teams::teams;
use async_graphql::SimpleObject;
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
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
