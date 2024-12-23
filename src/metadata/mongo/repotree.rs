use crate::api::dto::repo_dto::RepoTree;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize,Serialize,Debug,Clone,ToSchema)]
pub struct RepoTreeModel{
    pub hash: String,
    pub branch: String,
    pub owner: String,
    pub repo: String,
    pub time: i64,
    pub tree: RepoTree 
}