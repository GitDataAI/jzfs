use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone, ToSchema)]
pub struct GraphQLRepoIssuesDto{
    pub owner: String,
    pub repo: String,
    pub page: u64,
    pub size: i64,
    pub issues_id: Option<Uuid>,
}