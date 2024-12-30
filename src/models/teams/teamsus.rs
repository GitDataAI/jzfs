use uuid::Uuid;

use crate::models::users::users;
use sea_orm::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "teamus")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub join_at: i64,
    pub access: i64,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    TeamModel,
    UserModel,
}

impl Related<super::teams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamModel.def()
    }
}

impl Related<users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserModel.def()
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::TeamModel => Entity::belongs_to(super::teams::Entity)
                .from(Column::TeamId)
                .to(super::teams::Column::Uid)
                .into(),
            Relation::UserModel => Entity::belongs_to(users::Entity)
                .from(Column::UserId)
                .to(users::Column::Uid)
                .into(),
        }
    }
}
