use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::metadata::model::groups::groups;

#[derive(Deserialize, ToSchema)]
pub struct GroupCreate{
    pub name: String,
    pub contact: String,
    pub description: String,
}

#[derive(Deserialize, ToSchema)]
pub struct GroupQuery{
    pub key: String,
    pub page: u64,
    pub size: u64,
}


#[derive(Deserialize, ToSchema, Serialize)]
pub struct GroupDesc{
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub avatar: Option<String>,
    pub status: i32,
    pub website: Vec<String>,
    pub company: String,
    pub description: Option<String>,
    pub localtime: String,
    pub timezone: String,
    pub pro: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<&groups::Model> for GroupDesc {
    fn from(value: &groups::Model) -> Self {
        Self{
            uid: value.uid,
            name: value.name.clone(),
            username: value.username.clone(),
            avatar: value.avatar.clone(),
            status: value.status,
            website: value.website.clone(),
            company: value.company.clone(),
            description: value.description.clone(),
            localtime: value.localtime.clone(),
            timezone: value.timezone.clone(),
            pro: value.pro,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
#[derive(Deserialize,Serialize,Debug,Clone,ToSchema)]
pub struct GroupsLabels{
    pub labels: String,
    pub color: String,
}