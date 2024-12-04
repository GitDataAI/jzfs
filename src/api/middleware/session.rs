use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use crate::metadata::model::users::users::Model;
pub const ALLOW_NEXT_KEY: &str = "allow_next";
pub const CAPTCHA: &str = "captcha";
pub const SESSION_USER_KEY: &str = "session_user";
#[derive(Deserialize,Serialize,Clone,Debug, ToSchema)]
pub struct SessionModel{
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub public_email: bool,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub status: i32,

    pub theme: String,

    pub pro: bool,
}

impl From<Model> for SessionModel{
    fn from(value: Model) -> Self {
        Self{
            uid: value.uid,
            name: value.name,
            username: value.username,
            email: value.email,
            public_email: value.public_email,
            avatar: value.avatar,
            phone: value.phone,
            status: value.status,
            theme: value.theme,
            pro: value.pro,
        }
    }
}