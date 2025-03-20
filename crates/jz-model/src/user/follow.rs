use sea_orm::prelude::{DateTime, Uuid};
use sea_orm::{ActiveValue, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "follow")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub target_id: Uuid,
    pub created_at: DateTime,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        ActiveModel {
            uid: ActiveValue::Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
