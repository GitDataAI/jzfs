use sea_orm::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;


#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "commit")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub branch_id: Uuid,

    pub bio: String,

    pub commit_user: String,
    pub commit_email: String,
    pub commit_user_id: Uuid,

    pub commit_id: i64,

    pub created_at: OffsetDateTime,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
