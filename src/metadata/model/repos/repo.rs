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
    
    pub commit: i64,
    pub head_hash: String,
    
    pub ssh_path: String,
    pub http_path: String,

    pub star: i64,

    pub fork: i64,
    pub is_fork: bool,
    pub fork_from: Option<Uuid>,
    
    pub watch: i64,
    
    pub issue: i64,
    pub open_issue: i64,
    pub close_issue: i64,
    
    pub pr: i64,
    pub open_pr: i64,
    pub close_pr: i64,
    
    pub is_empty: bool,
    pub visible: bool,
    
    pub topic: Vec<String>,
    
    pub size: f64,
    
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub created_by: Uuid,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}