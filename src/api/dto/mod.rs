use serde::Deserialize;
use utoipa::ToSchema;

pub mod user_dto;
pub mod repo_dto;
pub mod email_dto;
pub mod groups_dto;

#[derive(Deserialize,ToSchema)]
pub struct ListOption{
    pub page: u64,
    pub size: u64,
}