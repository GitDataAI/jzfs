use crate::models::groups::groups;
use crate::models::users::users;
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "teaminv")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub team_id: Uuid,
    pub token: String,
    pub origin: Uuid,
    pub expire: i64,
    pub access: i64,
    pub created: i64,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    TeamModel,
    UserModel,
    GroupModel,
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
                .from(Column::Origin)
                .to(users::Column::Uid)
                .into(),
            Relation::GroupModel => Entity::belongs_to(groups::Entity)
                .from(Column::Origin)
                .to(groups::Column::Uid)
                .into(),
        }
    }
}
