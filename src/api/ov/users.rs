use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::metadata::model::users::{users, users_email, users_other};
use crate::metadata::model::users::users::Model;

#[derive(Deserialize,Serialize)]
pub struct UserOv{
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub phone: Option<String>,
    pub theme: String,
    pub sex: Option<String>,
    pub website: Vec<String>,
    pub company: String,
    pub description: String,
    pub localtime: String,
    pub timezone: String,
}
impl From<Model> for UserOv {
    fn from(value: Model) -> Self {
        Self{
            uid: value.uid,
            name: value.name,
            username: value.username,
            phone: value.phone,
            theme: value.theme,
            sex: value.sex,
            website: value.website,
            company: value.company,
            description: value.description,
            localtime: value.localtime,
            timezone: value.timezone,
        }
    }
}

#[derive(Deserialize,Serialize)]
pub struct UserFollowerOv {
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub avatar: Option<String>,
    pub description: String,
}
