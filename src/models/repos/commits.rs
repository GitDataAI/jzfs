use crate::models::users::email;
use sea_orm::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "commits")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub branch_id: Uuid,
    pub description: String,
    pub commit_user: String,
    pub commit_email: String,
    pub commit_id: String,
    pub created: i64,
}
impl ActiveModelBehavior for ActiveModel {}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RepoModel,
    BranchModel,
    EmailModel,
}
impl Related<super::repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RepoModel.def()
    }
}
impl Related<super::branchs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BranchModel.def()
    }
}
impl Related<email::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EmailModel.def()
    }
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::RepoModel => Entity::belongs_to(super::repos::Entity)
                .from(Column::RepoId)
                .to(super::repos::Column::Uid)
                .into(),
            Relation::BranchModel => Entity::belongs_to(super::branchs::Entity)
                .from(Column::BranchId)
                .to(super::branchs::Column::Uid)
                .into(),
            Relation::EmailModel => Entity::belongs_to(email::Entity)
                .from(Column::CommitEmail)
                .to(email::Column::Content)
                .into(),
        }
    }
}
