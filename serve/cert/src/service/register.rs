use chrono::Utc;
use crate::service::AppCertService;
use sea_orm::*;
use sha256::Sha256Digest;
use uuid::Uuid;
use crate::models::users;
use crate::rpc::session::UsersSession;
use crate::schema::{AppResult, CertRegisterParam};

impl AppCertService {
    pub async fn auth_user_register(&self, param: CertRegisterParam) -> AppResult<UsersSession> {
        match users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Email.eq(&param.email))
                    .add(users::Column::Username.eq(&param.username))
            )
            .one(&self.db)
            .await {
            Ok(Some(_)) => {
                return AppResult{
                    code: 400,
                    data: None,
                    msg: Some("Registration failed: User already exists".to_string()),
                }
            }
            _ => {}
        }
        let user = users::ActiveModel {
            uid: Set(Uuid::now_v7()),
            username: Set(param.username),
            password: Set(param.password.digest()),
            email: Set(param.email),
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
            max_repository: Set(1000),
            max_team: Set(100),
            max_group: Set(100),
            max_project: Set(100),
            show_email: Set(false),
            show_active: Set(true),
            show_project: Set(true),
            can_search: Set(true),
            can_follow: Set(true),
            theme: Set("System".to_string()),
            created_at: Set(Utc::now().naive_local()),
            updated_at: Set(Utc::now().naive_local()),
            deleted_at: Set(None),
            last_login_at: Set(Some(Utc::now().naive_local())),
        };
        if let Ok(model) = user.insert(&self.db).await {
            AppResult {
                code: 200,
                data: Some(UsersSession::from(model)),
                msg: Some("Register success".to_string()),
            }
        } else {
            AppResult {
                code: 500,
                data: None,
                msg: Some("Registration failed: Please try again later".to_string()),
            }
        }
    }
}