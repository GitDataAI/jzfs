use sea_orm::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "repos")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub description: String,

    pub star: i64,
    pub fork: i64,
    pub watch: i64,
    pub issue: i64,
    pub pr: i64,
    pub commit: i64,

    pub tag: Vec<String>,
    pub starer: Vec<Uuid>,
    pub watcher: Vec<Uuid>,

    pub visible: bool,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}