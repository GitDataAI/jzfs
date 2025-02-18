use sea_orm::prelude::{DateTime, Uuid};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::model::repository::repository;
use crate::model::users::{follow, ssh, star, tokens};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel,Serialize,Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    
    
    pub description: Option<String>,
    pub website: Option<String>,
    pub avatar: Option<String>,
    
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub theme: Option<String>,
    pub location: Option<String>,
    pub topic: Vec<String>,
    
    pub setting: Vec<String>,
    
    pub active: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Repository,
    Token,
    Follow,
    Ssh,
    Star
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Repository => Entity::has_many(repository::Entity).into(),
            Self::Token => Entity::has_many(tokens::Entity).into(),
            Self::Follow => Entity::has_many(follow::Entity).into(),
            Self::Ssh => Entity::has_many(ssh::Entity).into(),
            Self::Star => Entity::has_many(star::Entity).into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}