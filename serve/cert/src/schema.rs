use anyhow::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct AppResult<T> {
    pub code: i32,
    pub data: Option<T>,
    pub msg: Option<String>,
}

impl<T> From<anyhow::Error> for AppResult<T> {
    fn from(value: Error) -> Self {
        AppResult {
            code: 500,
            data: None,
            msg: Some(value.to_string()),
        }
    }
}

pub fn result_ok() -> AppResult<()> {
    AppResult {
        code: 200,
        data: None,
        msg: None,
    }
}

pub fn result_ok_with_data<T>(data: T) -> AppResult<T> {
    AppResult {
        code: 200,
        data: Some(data),
        msg: None,
    }
}
pub fn result_ok_with_msg(msg: String) -> AppResult<()> {
    AppResult {
        code: 200,
        data: None,
        msg: Some(msg),
    }
}
pub fn result_error(code: i32, msg: String) -> AppResult<()> {
    AppResult {
        code,
        data: None,
        msg: Some(msg),
    }
}
pub fn result_error_with_msg(msg: String) -> AppResult<()> {
    AppResult {
        code: 500,
        data: None,
        msg: Some(msg),
    }
}
pub fn result_error_with_msg_data<T>(msg: String) -> AppResult<T> {
    AppResult {
        code: 500,
        data: None,
        msg: Some(msg),
    }
}

pub fn result_error_with_code(code: i32) -> AppResult<()> {
    AppResult {
        code,
        data: None,
        msg: None,
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CertAuthLoginParam {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CertRegisterParam {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CertEmailCaptchaParam {
    pub email: String,
    pub length: Option<usize>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CertEmailCaptchaVerify {
    pub email: String,
    pub captcha: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MqEmailCode {
    pub email: String,
    pub captcha: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SecurityEventRegisterParam {
    pub title: String,
    pub description: Option<String>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub device: Option<String>,
    pub location: Option<String>,
    pub action: String,
    pub actor: String,
    pub actor_uid: Uuid,
    pub user: String,
    pub user_uid: Uuid,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SshKeySearch {
    pub public_key: String,
    pub algorithm: Option<String>,
    pub comment: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AccessKeySearch {
    pub access_key: String,

    pub req_repo_access: i32,
    pub req_email_access: i32,
    pub req_event_access: i32,
    pub req_follow_access: i32,
    pub req_gpg_access: i32,
    pub req_ssh_access: i32,
    pub req_webhook_access: i32,
    pub req_wiki_access: i32,
    pub req_project_access: i32,
    pub req_issue_access: i32,
    pub req_comment_access: i32,
    pub req_profile_access: i32,
}
