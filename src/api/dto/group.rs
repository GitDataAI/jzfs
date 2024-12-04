use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct GroupCreate{
    pub name: String,
    pub contact: String,
    pub description: String,
}