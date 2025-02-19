use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel,Serialize,Deserialize)]
#[sea_orm(table_name = "statistics_repo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_uid: Uuid,
    pub rtype: String, // star fork watch click
    pub days: i64,
    pub mount: i64,
    pub years: i64,
    pub count: i64,
}

#[derive(Copy, Clone, Debug, EnumIter,DeriveRelation)]
pub enum Relation {
}


impl ActiveModelBehavior for ActiveModel {}
