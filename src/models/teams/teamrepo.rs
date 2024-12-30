use crate::models::repos::repos;
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "teamrepo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub team_id: Uuid,
    pub access: i64,
    pub created: i64,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RepoModel,
    TeamModel,
}

impl Related<repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RepoModel.def()
    }
}

impl Related<super::teams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamModel.def()
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::RepoModel => Entity::belongs_to(repos::Entity)
                .from(Column::RepoId)
                .to(repos::Column::Uid)
                .into(),
            Relation::TeamModel => Entity::belongs_to(super::teams::Entity)
                .from(Column::TeamId)
                .to(super::teams::Column::Uid)
                .into(),
        }
    }
}
