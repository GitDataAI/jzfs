use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::users::users;

pub const USER_SESSION_KEY: &'static str = "users_session_model";

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct UsersSessionModel {
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub theme: String,
    pub pinned: Vec<Uuid>,
    pub main_email: String,
    pub pro: bool,
    pub avatar_url: Option<String>,
}

impl From<users::Model> for UsersSessionModel {
    fn from(model: users::Model) -> Self {
        UsersSessionModel {
            uid: model.uid,
            name: model.name,
            username: model.username,
            theme: model.theme,
            pinned: model.pinned,
            main_email: model.main_email,
            pro: model.pro,
            avatar_url: model.avatar_url,
        }
    }
}

impl From<&users::Model> for UsersSessionModel {
    fn from(model: &users::Model) -> Self {
        UsersSessionModel {
            uid: model.uid,
            name: model.name.clone(),
            username: model.username.clone(),
            theme: model.theme.clone(),
            pinned: model.pinned.clone(),
            main_email: model.main_email.clone(),
            pro: model.pro,
            avatar_url: model.avatar_url.clone(),
        }
    }
}