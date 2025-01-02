use serde::Deserialize;
use utoipa::ToSchema;

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
#[derive(Deserialize, ToSchema)]
pub struct UserUpdate{
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub company: Option<String>,
    pub website: Option<Vec<String>>,
    pub sex: Option<String>,
    pub description: Option<String>,
}

