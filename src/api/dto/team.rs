use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct TeamCreate{
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, ToSchema)]
pub struct TeamInvite{
    pub email: String,
}