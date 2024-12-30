use crate::models::groups::groups;
use crate::models::teams::teamrepo;
use async_graphql::SimpleObject;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "teams")]
#[graphql(name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created: i64,
    pub updated: i64,
    pub created_by: Uuid,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    OrganizationModel,
    TeamRepo,
}

impl Related<groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrganizationModel.def()
    }
}
impl Related<teamrepo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamRepo.def()
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::OrganizationModel => Entity::belongs_to(groups::Entity)
                .from(Column::OrgId)
                .to(groups::Column::Uid)
                .into(),
            Relation::TeamRepo => Entity::has_many(teamrepo::Entity).into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamCreateOption {
    pub org_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
