use sea_orm::prelude::{DateTime, Uuid};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::model::users::users;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel,Serialize,Deserialize)]
#[sea_orm(table_name = "repository")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    pub uid: Uuid,
    pub name: String,
    pub description: Option<String>,
    
    pub owner_id: Uuid,
    
    pub visibility: bool,
    pub fork: Option<Uuid>,
    
    pub default_branch: String,
    pub node_uid: Uuid,
    pub avatar: Option<String>,
    
    pub nums_fork: i32,
    pub nums_star: i32,
    pub nums_watch: i32,
    pub nums_issue: i32,
    pub nums_pullrequest: i32,
    pub nums_commit: i32,
    pub nums_release: i32,
    pub nums_tag: i32,
    pub nums_branch: i32,
    
    pub ssh: String,
    pub http: String,
    
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub created_by: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Owner,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Owner => Entity::belongs_to(users::Entity)
                .from(Column::OwnerId)
                .to(users::Column::Uid)
                .into(),
        }
    }
}

impl Related<users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}