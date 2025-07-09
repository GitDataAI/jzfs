use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UsersFollowLink {
    pub target_uid: Uuid,
    pub user_uid: Uuid,
    pub special: bool,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UserFollowItem {
    pub username: String,
    pub uid: Uuid,
    pub description: Option<String>,
    pub avatar: Option<String>,
    pub created_at: DateTime,
    pub special: bool,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UserFollowCount {
    pub following_count: u64,
    pub followed_count: u64,
}


#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UserBlackListItem {
    pub user_id: Uuid,
    pub target_id: Uuid,
    pub description: Option<String>,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UserBasicFromParam {
    pub description: Option<String>,
    pub website: Vec<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub location: Option<String>,
}


#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UserSshKeyCreate {
    pub name: String,
    pub description: Option<String>,
    pub content: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UserSshKeyDelete {
    pub ssh_key_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UserAccessTokenCreate {
    pub title: String,
    pub description: Option<String>,
    pub expiration: String,
    pub repo_access: i32,
    pub email_access: i32,
    pub event_access: i32,
    pub follow_access: i32,
    pub gpg_access: i32,
    pub ssh_access: i32,
    pub webhook_access: i32,
    pub wiki_access: i32,
    pub project_access: i32,
    pub issue_access: i32,
    pub comment_access: i32,
    pub profile_access: i32,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UserAccessTokenItem {
    pub uid: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub expiration: String,
    pub repo_access: i32,
    pub email_access: i32,
    pub event_access: i32,
    pub follow_access: i32,
    pub gpg_access: i32,
    pub ssh_access: i32,
    pub webhook_access: i32,
    pub wiki_access: i32,
    pub project_access: i32,
    pub issue_access: i32,
    pub comment_access: i32,
    pub profile_access: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UserAccessTokenDelete {
    pub user_uid: Uuid,
    pub access_token_uid: Uuid,
}


#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UserCheckParam {
    pub username: Option<String>,
    pub email: Option<String>,
}