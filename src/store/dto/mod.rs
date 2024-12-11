use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct CommitDto{
    pub hash: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub date: String,
    pub branch: String,
    pub unix: i64,
    pub files: Vec<ObjectFile>,
}

#[derive(Deserialize,Serialize,ToSchema,Debug)]
pub struct ObjectFile{
    pub root: String,
    pub name: String,
    pub hash: String,
}

#[derive(Deserialize,Serialize,ToSchema,Debug)]
pub struct ConflictsFiles{
    pub ours: String,
    pub theirs: String,
}