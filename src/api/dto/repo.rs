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