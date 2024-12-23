use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use crate::api::dto::repo_dto::RepoTree;

#[derive(Deserialize,Serialize,Debug,Clone,ToSchema)]
pub struct RepoTreeModel{
    pub hash: String,
    pub branch: String,
    pub owner: String,
    pub repo: String,
    pub time: i64,
    pub tree: RepoTree 
}