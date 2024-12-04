use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct SearchTeamOptions{
    pub user_id: Uuid,
    pub keyword: String,
    pub group_id: Uuid,
    pub desc_include: String,
}