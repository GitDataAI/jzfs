// CREATE TABLE user_preferences (
// user_id UUID PRIMARY KEY,
// topics  TEXT[]      -- 用户手动订阅的 topic
// );

use sea_orm::entity::prelude::*;
use sea_orm::prelude::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_preferences")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: Uuid,
    pub topics: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
