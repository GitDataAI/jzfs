use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize,Serialize,ToSchema)]
pub struct GraphQLRepoQuery{
    pub owner: String,
    pub repo: String,
    pub profile: bool,
    pub data: bool,
    pub branchs: Option<GraphQLRepoBranchQuery>,
    pub tree: Option<GraphQLRepoTreeQuery>,
    pub license: bool,
    pub readme: Option<String>,
    pub contribute: bool,
}

#[derive(Deserialize,Serialize,ToSchema,Clone)]
pub struct GraphQLRepoCommitQuery{
    pub offset: u64,
    pub size: u64,
}
#[derive(Deserialize,Serialize,ToSchema,Clone)]
pub struct GraphQLRepoBranchQuery{
    pub commit: Option<GraphQLRepoCommitQuery>,
}

#[derive(Deserialize,Serialize,ToSchema,Clone)]
pub struct GraphQLRepoTreeQuery{
    pub branch: String,
    pub commit: Option<String>,
}