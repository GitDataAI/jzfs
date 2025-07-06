use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "access_key")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,

    pub title: String,
    pub description: Option<String>,
    pub name: String,
    #[serde(skip)]
    pub token: String,
    pub access: String,
    pub use_history: Vec<String>,

    pub resource_owner: String,
    pub resource_owner_uid: Uuid,

    pub expiration: String,

    pub fingerprint: String,

    // access 0 no 1 read 2 read and write
    pub repo_access: i32,

    pub email_access: i32,
    pub event_access: i32,
    pub follow_access: i32,
    pub gpg_access: i32,
    pub ssh_access: i32,
    pub webhook_access: i32,
    pub wiki_access: i32,
    pub project_access: i32,
    pub issue_access: i32,
    pub comment_access: i32,
    pub profile_access: i32,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
