use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::metadata::model::repos::repo;

#[derive(Deserialize,Serialize)]
pub struct RepoInfo{
    pub uid: Uuid,
    pub name: String,
    pub description: String,

    pub commit: i64,
    pub head_hash: String,


    pub star: i64,

    pub fork: i64,
    pub is_fork: bool,
    pub fork_from: Option<Uuid>,

    pub watch: i64,

    pub issue: i64,
    pub open_issue: i64,
    pub close_issue: i64,

    pub pr: i64,
    pub open_pr: i64,
    pub close_pr: i64,

    pub is_empty: bool,
    pub visible: bool,

    pub topic: Vec<String>,

    pub size: f64,
    
    pub license: Option<String>,
    pub contribute: Vec<Uuid>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub created_by: Uuid,
}

#[derive(Deserialize,Serialize)]
pub struct RepoBasicInfo{
    pub uid: Uuid,
    pub name: String,
    pub description: String,
    pub head_hash: String,
}

impl Into<RepoBasicInfo> for repo::Model {
    fn into(self) -> RepoBasicInfo {
        RepoBasicInfo{
            uid: self.uid,
            name: self.name,
            description: self.description,
            head_hash: self.head_hash,
        }
    }
}