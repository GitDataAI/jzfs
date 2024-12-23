use crate::metadata::mongo::repotree::RepoTreeModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize,Serialize,ToSchema)]
pub struct GraphQLRepoModel{
    pub owner: String,
    pub repo: String,
    pub profile: Option<GraphQLRepoProfile>,
    pub data: Option<GraphQLRepoData>,
    pub branchs: Option<Vec<GraphQLRepoBranchOv>>,
    pub tree: Option<RepoTreeModel>,
    pub license: Option<Vec<GraphQLRepoLicense>>,
    pub readme: Option<Vec<u8>>,
}

#[derive(Deserialize,Serialize,ToSchema)]
pub struct GraphQLRepoProfile{
    pub uid: Uuid,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub head_hash: Option<String>,
    pub ssh_path: String,
    pub http_path: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub visible: bool,
}

#[derive(Deserialize,Serialize,ToSchema)]
pub struct GraphQLRepoData{
    pub commit: i64,
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
    pub topic: Vec<String>,
    pub size: f64,
}

#[derive(Deserialize,Serialize,ToSchema,Clone)]
pub struct GraphQLRepoBranchOv{
    pub uid: Uuid,
    pub branch: String,
    pub protect: bool,
    pub visible: bool,
    pub head: Option<Uuid>,
    pub created_at: i64,
    pub updated_at: i64,
    pub commit: Vec<GraphQLRepoCommits>,
}

#[derive(Deserialize,Serialize,ToSchema,Clone)]
pub struct GraphQLRepoCommits{
    pub uid: Uuid,
    pub bio: String,
    pub commit_user: String,
    pub commit_email: String,
    pub commit_id: String,
    pub created_at: i64,
}

#[derive(Deserialize,Serialize,ToSchema)]
pub struct GraphQLRepoLicense{
    pub uid: Uuid,
    pub name: String,
    pub license: String,
    pub created_at: i64,
    pub updated_at: i64,
}