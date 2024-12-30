use serde::{Deserialize, Serialize};

pub mod avatar;
pub mod email;
pub mod followers;
pub mod following;
pub mod notifications;
pub mod profile;
pub mod repo;
pub mod ssh_key;
pub mod token;
pub mod options;

#[derive(Deserialize, Serialize)]
pub struct AvatarGet {
    pub url: String,
}
