use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTime;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::uuid_v7;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "notification")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub user_uid: Uuid,
    pub repo_uid: Option<Uuid>,
    pub issue_uid: Option<Uuid>,
    pub comment_uid: Option<Uuid>,
    pub replay_user_uid: Option<Uuid>,
    
    pub title: Option<String>,
    pub content: String,

    pub read: bool,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


impl ActiveModel {
    pub fn new_system(
        user_uid: Uuid,
        content: String,
    ) -> ActiveModel {
        ActiveModel {
            uid: Set(uuid_v7()),
            user_uid: Set(user_uid),
            repo_uid: Set(None),
            issue_uid: Set(None),
            comment_uid: Set(None),
            replay_user_uid: Set(None),
            title: Set(None),
            content: Set(content),
            read: Set(false),
            created_at: Set(Utc::now().naive_utc()),
        }
    }
    pub fn new_repo(
        user_uid: Uuid,
        repo_uid: Uuid,
        content: String,
    )
    -> ActiveModel {
        ActiveModel {
            uid: Set(uuid_v7()),
            user_uid: Set(user_uid),
            repo_uid: Set(Some(repo_uid)),
            issue_uid: Set(None),
            comment_uid: Set(None),
            replay_user_uid: Set(None),
            title: Set(None),
            content: Set(content),
            read: Set(false),
            created_at: Set(Utc::now().naive_utc()),
        }
    }
    pub fn new_comment(
        user_uid: Uuid,
        repo_uid: Uuid,
        issue_uid: Uuid,
        comment_uid: Uuid,
        replay_user_uid: Option<Uuid>,
        content: String,
    ) -> ActiveModel {
        ActiveModel {
            uid: Set(uuid_v7()),
            user_uid: Set(user_uid),
            repo_uid: Set(Some(repo_uid)),
            issue_uid: Set(Some(issue_uid)),
            comment_uid: Set(Some(comment_uid)),
            replay_user_uid: Set(replay_user_uid),
            title: Set(None),
            content: Set(content),
            read: Set(false),
            created_at: Set(Utc::now().naive_utc()),
        }
    }

    pub fn new_issue(
        user_uid: Uuid,
        repo_uid: Uuid,
        issue_uid: Uuid,
        content: String,
    ) -> ActiveModel {
        ActiveModel {
            uid: Set(uuid_v7()),
            user_uid: Set(user_uid),
            repo_uid: Set(Some(repo_uid)),
            issue_uid: Set(Some(issue_uid)),
            comment_uid: Set(None),
            replay_user_uid: Set(None),
            title: Set(None),
            content: Set(content),
            read: Set(false),
            created_at: Set(Utc::now().naive_utc()),

        }
    }
}