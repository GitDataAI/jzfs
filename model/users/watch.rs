use sea_orm::prelude::{DateTime, Uuid};
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Deserialize,Serialize)]
#[sea_orm(table_name = "watch")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub repository_id: Uuid,
    pub level: i32,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}



impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        ActiveModel {
            uid: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}