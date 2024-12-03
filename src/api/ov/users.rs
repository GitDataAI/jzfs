use serde::{Deserialize, Serialize};
use crate::metadata::model::users::users::Model;
use uuid::Uuid;

#[derive(Deserialize,Serialize)]
pub struct UserOv{
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub public_email: bool,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub status: i32,
    pub theme: String,
    pub pro: bool,
    pub team: Vec<Uuid>,
    pub repo: Vec<Uuid>,
    pub project: Vec<Uuid>,
    pub issue: Vec<Uuid>,
    pub pr: Vec<Uuid>,
    pub commit: Vec<Uuid>,
    pub tag: Vec<Uuid>,
    pub star: Vec<Uuid>,
    pub follow: Vec<Uuid>,
    pub sex: Option<String>,
    pub website: Vec<String>,
    pub company: String,
    pub description: String,

    pub localtime: String,
    pub timezone: String,
    
}
impl From<Model> for UserOv {
    fn from(value: Model) -> Self {
        Self{
            uid: value.uid,
            name: value.name,
            username: value.username,
            email: value.email,
            public_email: value.public_email,
            avatar: value.avatar,
            phone: value.phone,
            status: value.status,
            theme: value.theme,
            pro: value.pro,
            team: value.team,
            repo: value.repo,
            project: value.project,
            issue: value.issue,
            pr: value.pr,
            commit: value.commit,
            tag: value.tag,
            star: value.star,
            follow: value.follow,
            sex: value.sex,
            website: value.website,
            company: value.company,
            description: value.description,
            localtime: value.localtime,
            timezone: value.timezone,
        }
    }
}