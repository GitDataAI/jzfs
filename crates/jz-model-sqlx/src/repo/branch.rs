use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Clone,Debug,FromRow)]
pub struct BranchModel {
    pub uid: Uuid,
    pub repo_uid: Uuid,
    pub protect: bool,
    pub name: String,
    pub head: String,
    pub time: String,
}