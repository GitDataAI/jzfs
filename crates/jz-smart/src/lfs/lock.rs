use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct LfsLockParam {
    pub path: String,
    #[serde(rename = "ref")]
    pub r#ref: LfsLockParamRef,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct LfsLockParamRef {
    pub name: String,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct LfsLockResult {
    pub lock: LfsLockCreated,
}


#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct LfsLockCreated {
    pub id: Uuid,
    pub path: String,
    pub locked_at: String,
    pub owner: LfsLockParamRef
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct LfsLockError {
    pub message: String,
    pub request_id: Option<i32>,
    pub documentation_url: Option<String>,
    pub lock: LfsLockCreated
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct LfsLockListItem {
    pub locks: Vec<LfsLockCreated>,
    pub next_cursor: Option<String>
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct LfsLockVerifyParam {
    pub cursor: Option<String>,
    pub limit: u64,
    #[serde(rename = "ref")]
    pub r#ref: LfsLockParamRef
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct LfsLockVerifyResult {
    pub ours: Vec<LfsLockCreated>,
    pub theirs: Vec<LfsLockCreated>,
    pub next_cursor: Option<String>
}


#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct LfsUnLockParam {
    pub force: bool,
    #[serde(rename = "ref")]
    pub r#ref: LfsLockParamRef
}
pub async fn lfs_lock() -> impl Responder {
    return HttpResponse::Ok()
        .insert_header(("Content-Type", "application/vnd.git-lfs+json"))
        .json(json!({}));
}

pub async fn lfs_unlock() -> impl Responder {
    return HttpResponse::Ok()
        .insert_header(("Content-Type", "application/vnd.git-lfs+json"))
        .json(json!({}));
}

pub async fn lfs_lock_verify() -> impl Responder {
    let mut value = serde_json::Value::Null;
    value["our"] = json!({});
    value["other"] = json!({});
    return HttpResponse::Ok()
        .insert_header(("Content-Type", "application/vnd.git-lfs+json"))
        .json(value);
}

pub async fn lfs_lock_list() -> impl Responder {
    return HttpResponse::Ok()
        .insert_header(("Content-Type", "application/vnd.git-lfs+json"))
        .json(json!({}));
}

