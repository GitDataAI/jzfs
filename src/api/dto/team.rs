use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct TeamCreate{
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, ToSchema)]
pub struct TeamInvite{
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
pub struct TeamUid{
    pub(crate) uid: Uuid
}