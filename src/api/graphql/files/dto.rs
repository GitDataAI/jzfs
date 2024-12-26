use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize,Serialize,ToSchema)]
pub struct GraphQLFileDto{
    pub owner: String,
    pub repo: String,
    pub hash: Option<String>,
    pub branch: String,
    pub path: String,
    pub size_limit: i32,
    pub block: i32,
}

#[derive(Deserialize,Serialize,ToSchema)]
pub struct GraphQLFileOV{
    pub total: i32,
    pub current: i32,
    pub size: i32,
    pub data: Vec<u8>
}