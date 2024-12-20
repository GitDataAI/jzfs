use crate::api::dto::repo_dto::RepoBranchOv;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize,Serialize, ToSchema)]
pub struct GraphQLUserModel{
    pub profile: Option<GraphQLUserProfile>,
    pub repo: Option<Vec<GraphQLUserRepo>>,
    pub keys: Option<Vec<GraphQLUserKeys>>,
    pub data: Option<GraphQLUserData>,
    pub email: Option<Vec<GraphQLEmail>>,
    pub group: Option<Vec<GraphQLUserGroup>>
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct GraphQLUserProfile{
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub status: i32,
    pub website: Vec<String>,
    pub company: String,
    pub description: Option<String>,
    pub localtime: String,
    pub timezone: String,
    pub theme: String,
    pub pro: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub lastlogin: i64,
    pub is_groups: bool,
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct GraphQLUserRepo{
    pub uid: Uuid,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub branch: Vec<RepoBranchOv>,
    pub commit: i64,
    pub head_hash: Option<String>,
    pub ssh_path: String,
    pub http_path: String,
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
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct GraphQLUserKeys{
    pub uid: Uuid,
    pub created_at: String,
    pub head: String,
    pub last_use: String,
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct GraphQLUserData{
    pub uid: Uuid,
    pub repo: Vec<Uuid>,
    pub project: Vec<Uuid>,
    pub issue: Vec<Uuid>,
    pub pr: Vec<Uuid>,
    pub commit: Vec<Uuid>,
    pub tag: Vec<Uuid>,
    pub star: Vec<Uuid>,
    pub follow: Vec<Uuid>,
    pub following: Vec<Uuid>,
    pub watcher: Vec<Uuid>
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct GraphQLEmail{
    pub uid: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub is_public: bool,
    pub verified: bool,
    pub bind_at: String,
}


#[derive(Deserialize,Serialize, ToSchema)]
pub struct GraphQLUserGroup{
    pub name: String,
    pub username: String,
    pub theme: String,
    pub website: Vec<String>,
    pub company: String,
    pub description: Option<String>,
    pub localtime: String,
    pub timezone: String,
}