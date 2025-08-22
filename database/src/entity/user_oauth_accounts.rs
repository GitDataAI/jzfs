use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "oauth_providers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_uid: Uuid,
    pub provider_uid: Uuid,
    pub provider_user_id: Option<String>,
    pub provider_username: Option<String>,
    pub provider_email: Option<String>,
    pub provider_avatar_url: Option<String>,

    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
    pub raw_user_data: Json,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
