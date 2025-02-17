use sea_orm::prelude::{DateTime, Uuid};
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel,Deserialize,Serialize)]
#[sea_orm(table_name = "follow")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub target_id: Uuid,
    pub created_at: DateTime,
}


#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Users,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users => Entity::belongs_to(super::users::Entity)
                .from(Column::UserId)
                .to(super::users::Column::Uid)
                .into(),
        }
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        ActiveModel {
            uid: ActiveValue::Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}