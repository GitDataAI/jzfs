use uuid::Uuid;

use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "groups_repo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub group_id: Uuid,
    pub repo_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}