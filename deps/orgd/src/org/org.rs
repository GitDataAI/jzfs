use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTime;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "org")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,

    pub email: String,

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

    pub created_by: Uuid,
    pub owner_org: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


impl ActiveModel {
    pub fn new(
        name: String,
        email: String,
        description: Option<String>,
        created_by: Uuid,
        owner_org: Option<String>,
    )
        -> Self {
        ActiveModel {
            uid: Set(Uuid::now_v7()),
            name: Set(name),
            email: Set(email),
            description: Set(description),
            website: Set(None),
            avatar: Set(None),
            timezone: Set(None),
            language: Set(None),
            theme: Set(None),
            location: Set(None),
            topic: Set(vec![]),
            setting: Set(vec![]),
            active: Set(true),
            created_at: Set(chrono::Local::now().naive_utc()),
            updated_at: Set(chrono::Local::now().naive_utc()),
            created_by: Set(created_by),
            owner_org: Set(owner_org),
        }
    }
}

