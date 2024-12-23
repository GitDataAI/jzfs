use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize,Serialize,ToSchema)]
pub struct GraphQLTreeDto {
    pub owner: String,
    pub repo: String,
    pub hash: Option<String>,
    pub branch: String,
}