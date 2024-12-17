use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use crate::metadata::model::users::Users;

#[derive(Deserialize, ToSchema)]
pub struct UsersInner{
    pub inner: String,
}
#[derive(Deserialize, ToSchema)]
pub struct UsersLoginEmail{
    pub email: String,
    pub passwd: String,
}
#[derive(Deserialize, ToSchema)]
pub struct UsersLoginUsername{
    pub username: String,
    pub passwd: String,
}
#[derive(Deserialize, ToSchema)]
pub struct UserApply{
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, ToSchema)]
pub struct UserResetPassword{
    pub email: String,
    pub token: String,
    pub password: String,
}
#[derive(Deserialize, ToSchema)]
pub struct UserResetPasswd {
    pub old_password: String,
    pub new_password: String
}
#[derive(Deserialize, ToSchema, Clone)]
pub struct UserUpdate{
    pub name: Option<String>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub theme: Option<String>,
    pub website: Option<Vec<String>>,
    pub company: Option<String>,
    pub description: Option<String>,
    pub localtime: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Serialize, ToSchema,Clone)]
pub struct UserKeyList{
    pub uid: Uuid,
    pub created_at: String,
    pub head: String,
    pub last_use: String,
}

#[derive(Serialize,Deserialize,ToSchema)]
pub struct UserKeyCreate{
    pub name: String,
    pub pubkey: String,
}

#[derive(Serialize,Deserialize,ToSchema)]
pub struct UserAvatar{
    pub(crate) byte: Vec<u8>,
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct UserOv{
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub phone: Option<String>,
    pub theme: String,
    pub website: Vec<String>,
    pub company: String,
    pub description: Option<String>,
    pub localtime: String,
    pub timezone: String,
}
impl From<Users> for UserOv {
    fn from(value: Users) -> Self {
        Self{
            uid: value.uid,
            name: value.name,
            username: value.username,
            phone: value.phone,
            theme: value.theme,
            website: value.website,
            company: value.company,
            description: value.description,
            localtime: value.localtime,
            timezone: value.timezone,
        }
    }
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct UserFollowerOv {
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub avatar: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize,Serialize, ToSchema)]
pub struct UserFollow{
    pub uid: Uuid
}