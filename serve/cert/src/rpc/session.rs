use authd::users;
use authd::users::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UsersSession {
    pub uid: Uuid,
    pub username: String,
    pub avatar: Option<String>,
    pub email: String,
    pub status: Option<String>,
    pub ip: Option<String>,
    // Ip Owner Place
    pub localhost: Option<String>,
}

impl From<users::Model> for UsersSession {
    fn from(value: Model) -> Self {
        Self {
            uid: value.uid,
            username: value.username,
            avatar: value.avatar,
            email: value.email,
            status: None,
            ip: None,
            localhost: None,
        }
    }
}
