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
