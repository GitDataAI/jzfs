use crate::uuid_v7;
use chrono::Utc;
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub fingerprint: String,
    pub description: Option<String>,
    #[serde(skip)]
    pub token: String,
    pub access: String,
    pub use_history: Vec<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub expires_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(
        users_id: Uuid,
        name: String,
        fingerprint: String,
        description: Option<String>,
        token: String,
        access: String,
        expires_at: DateTime,
    ) -> ActiveModel {
        ActiveModel {
            uid: Set(uuid_v7()),
            user_id: Set(users_id),
            name: Set(name),
            fingerprint: Set(fingerprint),
            description: Set(description),
            token: Set(token),
            access: Set(access),
            use_history: Set(Vec::new()),
            created_at: Set(Utc::now().naive_local()),
            updated_at: Set(Utc::now().naive_local()),
            expires_at: Set(expires_at),
        }
    }
}
