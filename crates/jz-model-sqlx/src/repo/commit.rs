use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Clone,Debug,FromRow)]
pub struct CommitBranch {
    pub uid: Uuid,
    pub id: String,
    pub branch_uid: Uuid,
    pub repo_uid: Uuid,
    pub branch_name: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub status: String,
    pub runner: Vec<String>,
    pub time: String,
}