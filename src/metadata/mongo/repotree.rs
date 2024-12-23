use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::dto::repo_dto::RepoTree;

#[derive(Deserialize,Serialize,Debug)]
pub struct RepoTreeModel{
    pub uid: Uuid,
    pub hash: String,
    pub branch: String,
    pub owner: String,
    pub repo: String,
    pub tree: RepoTree 
}