use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize,Serialize, ToSchema)]
pub struct RepoCreate{
    pub owner: Uuid,
    pub is_group: bool,
    pub name: String,
    pub description: String,
    pub license_name: Option<String>,
    pub license: Option<String>,
    pub topic: Option<Vec<String>>,
    pub visible: bool,
    pub default_branch: String,
}

#[derive(Deserialize,Serialize,ToSchema)]
pub struct RepoSearch{
    pub keywords: String,
    pub page: u64,
    pub size: u64,
}
#[derive(Deserialize,Serialize,ToSchema)]
pub struct RepoInfo{
    pub owner: Uuid,
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct RepoBranchNew{
    pub from: String,
    pub branch: String,
    pub protect: bool,
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct RepoBranchDel{
    pub branch: String,
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct RepoBranchRename{
    pub branch: String,
    pub new_branch: String,
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct RepoBranchProtect{
    pub branch: String,
    pub protect: bool,
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct RepoBranchMerge{
    pub branch: String,
    pub target: String,
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct RepoFilePath{
    pub path: String,
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct RepoRename{
    pub name: String,
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct RepoTopic{
    pub topic: String
}



#[derive(Deserialize,Serialize, ToSchema)]
pub struct RepoTree{
    pub name: String,
    pub is_dir: bool,
    pub path: String,
    pub children: Vec<RepoTree>
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct RepoTreeQuery{
    pub commit: Option<String>
}