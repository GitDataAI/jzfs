use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "comment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uuid: Uuid,
    pub issue_uid: Uuid,
    pub comment_uid: Uuid,
    pub content: String,
    pub author_uid: Uuid,
    pub parent_comment_uid: Option<Uuid>, //父评论的ID，用于支持嵌套，若为顶级评论，则此值为空
    pub created_at: DateTimeUtc,
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
