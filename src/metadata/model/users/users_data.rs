use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "users_data")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_id: Uuid,
    pub repo: Vec<Uuid>,
    pub project: Vec<Uuid>,
    pub issue: Vec<Uuid>,
    pub pr: Vec<Uuid>,
    pub commit: Vec<Uuid>,
    pub tag: Vec<Uuid>,
    pub star: Vec<Uuid>,
    pub follow: Vec<Uuid>,
    pub following: Vec<Uuid>,
    pub watcher: Vec<Uuid>
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}