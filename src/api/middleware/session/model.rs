use crate::metadata::model::users::Users;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


pub const SessionModelKey: &str = "SessionModel";

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct SessionModel{
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub status: i32,
    pub theme: String,
    pub pro: bool,
    pub localtime: String,
    pub timezone: String,
}

impl From<Users> for SessionModel{
    fn from(value: Users) -> Self {
        Self{
            uid: value.uid,
            name: value.name,
            username: value.username,
            status: value.status,
            theme: value.theme,
            pro: value.pro,
            localtime: value.localtime,
            timezone: value.timezone,
        }
    }
}