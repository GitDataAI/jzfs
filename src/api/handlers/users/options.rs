use crate::error::{JZError, JZResult};
use anyhow::anyhow;
use base64::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PageParma {
    pub page: u64,
    pub size: u64,
}

#[derive(Deserialize, Serialize)]
pub struct Base64Inner {
    pub inner: String,
}
impl Base64Inner {
    pub fn decode<T>(&self) -> JZResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        match BASE64_STANDARD.decode(self.inner.as_bytes()) {
            Ok(data) => match serde_json::from_slice::<T>(&data) {
                Ok(data) => Ok(data),
                Err(err) => Err(JZError::Other(anyhow!("{}", err))),
            },
            Err(err) => Err(JZError::Other(anyhow!("{}", err))),
        }
    }
}
#[derive(Deserialize, Serialize)]
pub struct UsersLogin {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Serialize)]
pub struct UsersApply {
    pub username: String,
    pub password: String,
    pub email: String,
}
#[derive(Serialize, Deserialize)]
pub struct UserKeyCreate {
    pub name: String,
    pub pubkey: String,
    pub access: i16,
    pub expire: i64,
}
#[derive(Deserialize)]
pub struct EmailBind {
    pub email: String,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserFollowCount {
    pub following: usize,
    pub follower: usize,
}
