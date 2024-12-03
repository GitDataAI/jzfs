use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SearchTeamOptions{
    pub user_id: Uuid,
    pub keyword: String,
    pub group_id: Uuid,
    pub desc_include: String,
}