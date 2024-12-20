use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize,Serialize,ToSchema)]
pub struct UserGraphqlQuery{
    pub profile: bool,
    pub username: Option<String>,
    pub repo: bool,
    pub keys: bool,
    pub data: bool,
    pub email: bool,
    pub groups: bool,
}