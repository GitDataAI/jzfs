use crate::uuid_v7;
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub username: String,
    #[serde(skip)]
    pub password: String,
    pub email: String,

    pub description: Option<String>,
    pub avatar: Option<String>,
    pub website: Vec<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub location: Option<String>,

    pub nums_fans: i32,
    pub nums_following: i32,
    pub nums_projects: i32,
    pub nums_issues: i32,
    pub nums_comments: i32,
    pub nums_stars: i32,
    pub nums_teams: i32,
    pub nums_groups: i32,
    pub nums_repositories: i32,
    pub nums_reviews: i32,

    pub allow_use: bool,
    pub allow_create: bool,
    pub max_repository: i32,
    pub max_team: i32,
    pub max_group: i32,
    pub max_project: i32,

    pub show_email: bool,
    pub show_active: bool,
    pub show_project: bool,

    pub can_search: bool,
    pub can_follow: bool,

    pub theme: String,

    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
    pub last_login_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(username: String, password: String, email: String) -> Self {
        Self {
            uid: Set(uuid_v7()),
            username: Set(username),
            password: Set(password),
            email: Set(email),
            description: Set(None),
            avatar: Set(None),
            website: Set(vec![]),
            timezone: Set(None),
            language: Set(None),
            location: Set(None),
            nums_fans: Set(0),
            nums_following: Set(0),
            nums_projects: Set(0),
            nums_issues: Set(0),
            nums_comments: Set(0),
            nums_stars: Set(0),
            nums_teams: Set(0),
            nums_groups: Set(0),
            nums_repositories: Set(0),
            nums_reviews: Set(0),
            allow_use: Set(true),
            allow_create: Set(true),
            max_repository: Set(10),
            max_team: Set(10),
            max_group: Set(10),
            max_project: Set(10),
            show_email: Set(true),
            show_active: Set(true),
            show_project: Set(true),
            can_search: Set(true),
            can_follow: Set(true),
            theme: Set("default".to_string()),
            created_at: Set(chrono::Local::now().naive_local()),
            updated_at: Set(chrono::Local::now().naive_local()),
            deleted_at: Set(None),
            last_login_at: Set(None),
        }
    }
}
