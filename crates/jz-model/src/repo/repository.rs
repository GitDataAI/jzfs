use crate::uuid_v7;
use jz_git::GitParam;
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::{DateTime, Uuid};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "repository")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    pub uid: Uuid,
    pub name: String,
    pub description: Option<String>,

    pub owner_uid: Uuid,
    pub owner_name: String,

    pub website: Option<String>,
    pub project: Vec<Uuid>,

    pub is_private: bool,

    pub fork: Option<Uuid>,

    pub default_branch: String,

    pub nums_fork: i32,
    pub nums_star: i32,
    pub nums_watch: i32,
    pub nums_issue: i32,
    pub nums_pullrequest: i32,
    pub nums_commit: i32,
    pub nums_release: i32,
    pub nums_tag: i32,
    pub nums_branch: i32,

    pub topic: Vec<String>,
    pub status: String,
    pub rtype: String,
    pub node: Uuid,

    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub created_by: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn git(&self) -> anyhow::Result<GitParam> {
        let node = self.node.to_string();
        let uid = self.uid.to_string();
        GitParam::new(PathBuf::from("./data").join(node), uid)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RepositoryInitParam {
    pub owner_name: String,
    pub owner_uid: Uuid,
    pub repo_name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub topic: Vec<String>,
    pub rtype: String,
    pub default_branch: Option<String>,
    pub readme: bool,
    pub node: Uuid,
}

impl ActiveModel {
    pub fn new(param: RepositoryInitParam) -> ActiveModel {
        ActiveModel {
            uid: Set(uuid_v7()),
            name: Set(param.repo_name),
            description: Set(param.description),
            owner_uid: Set(param.owner_uid),
            owner_name: Set(param.owner_name),
            website: Set(None),
            project: Set(vec![]),
            is_private: Set(param.is_private),
            fork: Set(None),
            default_branch: Set(param.default_branch.unwrap_or("master".to_string())),
            nums_fork: Set(0),
            nums_star: Set(0),
            nums_watch: Set(0),
            nums_issue: Set(0),
            nums_pullrequest: Set(0),
            nums_commit: Set(0),
            nums_release: Set(0),
            nums_tag: Set(0),
            nums_branch: Set(0),
            topic: Set(param.topic),
            status: Set("active".to_string()),
            rtype: Set(param.rtype),
            created_at: Set(chrono::Local::now().naive_local()),
            updated_at: Set(chrono::Local::now().naive_local()),
            created_by: Set(param.owner_uid),
            node: Set(param.node),
        }
    }
}
