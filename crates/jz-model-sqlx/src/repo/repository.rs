use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Clone,Debug,FromRow)]
pub struct RepositoryModel {
    pub uid: Uuid,
    pub name: String,
    pub description: Option<String>,

    pub owner_uid: Uuid,
    pub owner_name: String,

    pub website: Option<String>,
    pub project: Vec<Uuid>,

    pub is_private: bool,

    pub fork: Option<Uuid>,

    pub default_branch: String,

    pub nums_fork: i32,
    pub nums_star: i32,
    pub nums_watch: i32,
    pub nums_issue: i32,
    pub nums_pullrequest: i32,
    pub nums_commit: i32,
    pub nums_release: i32,
    pub nums_tag: i32,
    pub nums_branch: i32,

    pub topic: Vec<String>,
    pub status: String,
    pub rtype: String,
    pub node: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
}