use std::hash::{Hash, Hasher};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel,Eq, PartialEq, Hash)]
#[sea_orm(table_name = "repo_branch")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub branch: String,

    pub protect: bool,
    pub visible: bool,

    pub head: Option<Uuid>, // 最近一次commit的uid

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,

    pub created_by: Uuid,

}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}

impl Eq for ActiveModel {}
impl Hash for ActiveModel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.clone().unwrap().to_string().hash(state);
    }
}