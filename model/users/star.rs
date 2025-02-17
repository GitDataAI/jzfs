use sea_orm::prelude::{DateTime, Uuid};
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
use crate::model::repository::repository;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Deserialize,Serialize)]
#[sea_orm(table_name = "stars")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub repository_id: Uuid,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Users,
    Repository,
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users => Entity::belongs_to(super::users::Entity)
                .from(Column::UserId)
                .to(super::users::Column::Uid)
                .into(),
            Self::Repository => Entity::belongs_to(repository::Entity)
                .from(Column::RepositoryId)
                .to(repository::Column::Uid)
                .into(),
        }
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}
impl Related<repository::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Repository.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        ActiveModel {
            uid: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}