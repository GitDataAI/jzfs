use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::follow::FollowModel;
use crate::secrets::SecretsModel;
use crate::ssh_key::SshKeyModel;
use crate::token::TokenModel;

#[derive(Deserialize,Serialize,Clone,Debug,FromRow)]
pub struct UsersModel {
    #[sqlx(primary_key)]
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
}

pub struct UserMapper {
    pub db: sqlx::PgPool,
}

impl UserMapper {
    pub async fn insert(&self, users: UsersModel) -> Result<UsersModel, sqlx::Error> {
        sqlx::query_as::<_, UsersModel>(
            "
        INSERT INTO users (
            uid, username, password, email, description, avatar, website, timezone, language, location,
            nums_fans, nums_following, nums_projects, nums_issues, nums_comments, nums_stars, nums_teams, nums_groups,
            nums_repositories, nums_reviews, allow_use, allow_create, max_repository, max_team, max_group, max_project,
            show_email, show_active, show_project, can_search, can_follow, theme, created_at, updated_at, deleted_at, last_login_at
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19,
            $20, $21, $22, $23, $24, $25, $26, $27, $28,
            $29, $30, $31, $32, $33, $34, $35, $36
        )
                ",
            )
            .bind(users.uid)
            .bind(users.username)
            .bind(users.password)
            .bind(users.email)
            .bind(users.description)
            .bind(users.avatar)
            .bind(users.website)
            .bind(users.timezone)
            .bind(users.language)
            .bind(users.location)
            .bind(users.nums_fans)
            .bind(users.nums_following)
            .bind(users.nums_projects)
            .bind(users.nums_issues)
            .bind(users.nums_comments)
            .bind(users.nums_stars)
            .bind(users.nums_teams)
            .bind(users.nums_groups)
            .bind(users.nums_repositories)
            .bind(users.nums_reviews)
            .bind(users.allow_use)
            .bind(users.allow_create)
            .bind(users.max_repository)
            .bind(users.max_team)
            .bind(users.max_group)
            .bind(users.max_project)
            .bind(users.show_email)
            .bind(users.show_active)
            .bind(users.show_project)
            .bind(users.can_search)
            .bind(users.can_follow)
            .bind(users.theme)
            .bind(users.created_at)
            .bind(users.updated_at)
            .bind(users.deleted_at)
            .bind(users.last_login_at)
            .fetch_one(&self.db)
            .await
    }

    pub fn query(&self) -> UserQuery {
        UserQuery {
            db: self.db.clone(),
        }
    }
    pub fn update(&self, users: UsersModel) -> UserUpdate {
        UserUpdate {
            db: self.db.clone(),
            users,
        }
    }
    pub fn relation(&self, users: UsersModel) -> UserRelation {
        UserRelation {
            db: self.db.clone(),
            users: users.clone(),
        }
    }
}

pub struct UserQuery {
    pub db: sqlx::PgPool,
}

impl UserQuery {
    pub async fn get_user_by_uid(&self, uid: Uuid) -> Result<UsersModel, sqlx::Error> {
        sqlx::query_as::<_, UsersModel>("SELECT * FROM users WHERE uid = $1")
            .bind(uid)
            .fetch_one(&self.db)
            .await
    }
    pub async fn get_user_by_username(&self, username: String) -> Result<UsersModel, sqlx::Error> {
        sqlx::query_as::<_, UsersModel>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(&self.db)
            .await
    }
    pub async fn get_user_by_email(&self, email: String) -> Result<UsersModel, sqlx::Error> {
        sqlx::query_as::<_, UsersModel>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&self.db)
            .await
    }
}

pub struct UserUpdate {
    pub db: sqlx::PgPool,
    pub users: UsersModel,
}

impl UserUpdate {
    pub async fn update_description(&self, description: String) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET description = $1 WHERE uid = $2")
            .bind(description)
            .bind(self.users.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_avatar(&self, avatar: String) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET avatar = $1 WHERE uid = $2")
            .bind(avatar)
            .bind(self.users.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_website(&self, website: Vec<String>) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET website = $1 WHERE uid = $2")
            .bind(website)
            .bind(self.users.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_timezone(&self, timezone: String) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET timezone = $1 WHERE uid = $2")
            .bind(timezone)
            .bind(self.users.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_language(&self, language: String) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET language = $1 WHERE uid = $2")
            .bind(language)
            .bind(self.users.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_location(&self, location: String) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET location = $1 WHERE uid = $2")
            .bind(location)
            .bind(self.users.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_theme(&self, theme: String) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET theme = $1 WHERE uid = $2")
            .bind(theme)
            .bind(self.users.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_password(&self, password: String) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET password = $1 WHERE uid = $2")
            .bind(password)
            .bind(self.users.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_email(&self, email: String) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET email = $1 WHERE uid = $2")
            .bind(email)
            .bind(self.users.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_allow_use(&self, allow_use: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET allow_use = $1 WHERE uid = $2")
            .bind(allow_use)
            .bind(self.users.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn update_last_login_at(&self) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET last_login_at = $1 WHERE uid = $2")
            .bind(chrono::Local::now().naive_local())
            .bind(self.users.uid)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}

pub struct UserRelation {
    pub db: sqlx::PgPool,
    pub users: UsersModel,
}

impl UserRelation {
    pub async fn tokens(&self) -> Result<Vec<TokenModel>, sqlx::Error> {
        sqlx::query_as::<_, TokenModel>("SELECT * FROM tokens WHERE user_id = $1")
            .bind(self.users.uid)
            .fetch_all(&self.db)
            .await
    }
    pub async fn ssh_keys(&self) -> Result<Vec<SshKeyModel>, sqlx::Error> {
        sqlx::query_as::<_, SshKeyModel>("SELECT * FROM ssh WHERE user_id = $1")
            .bind(self.users.uid)
            .fetch_all(&self.db)
            .await
    }
    pub async fn secrets(&self) -> Result<Vec<SecretsModel>, sqlx::Error> {
        sqlx::query_as::<_, SecretsModel>("SELECT * FROM security WHERE user_uid = $1")
            .bind(self.users.uid)
            .fetch_all(&self.db)
            .await
    }
    pub async fn following(&self) -> Result<Vec<FollowModel>, sqlx::Error> {
        sqlx::query_as::<_, FollowModel>("SELECT * FROM follow WHERE user_id = $1")
            .bind(self.users.uid)
            .fetch_all(&self.db)
            .await
    }
    pub async fn followed(&self) -> Result<Vec<FollowModel>, sqlx::Error> {
        sqlx::query_as::<_, FollowModel>("SELECT * FROM follow WHERE target_id = $1")
            .bind(self.users.uid)
            .fetch_all(&self.db)
            .await
    }
}

pub struct UserBuilder {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            username: None,
            email: None,
            password: None,
        }
    }
    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    pub fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }
    pub async fn build(&self, db: &sqlx::PgPool) -> Result<UsersModel, sqlx::Error> {
        if self.username.is_none() || self.email.is_none() || self.password.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }
        let Some(username) = self.username.clone() else {
            return Err(sqlx::Error::RowNotFound);
        };
        let Some(email) = self.email.clone() else {
            return Err(sqlx::Error::RowNotFound);
        };
        let Some(password) = self.password.clone() else {
            return Err(sqlx::Error::RowNotFound);
        };
        if username.len() < 3 || username.len() > 20 {
            return Err(sqlx::Error::InvalidArgument(String::from("username length must be between 3 and 20")))
        }
        if email.len() < 3 || email.len() > 50 {
            return Err(sqlx::Error::InvalidArgument(String::from("email length must be between 3 and 50")))
        }
        if password.len() < 6 || password.len() > 20 {
            return Err(sqlx::Error::InvalidArgument(String::from("password length must be between 6 and 20")))
        }  
        let mapper = UserMapper{
            db: db.clone(),
        };
        let model = UsersModel {
            uid: Uuid::new_v4(),
            username: username.clone(),
            email: email.clone(),
            password: password.clone(),
            created_at: chrono::Local::now().to_utc(),
            updated_at: chrono::Local::now().to_utc(),
            allow_use: true,
            allow_create: false,
            max_repository: 100,
            max_team: 100,
            max_group: 5,
            max_project: 100,
            show_email: true,
            show_active: true,
            show_project: true,
            can_search: true,
            last_login_at: Some(chrono::Local::now().to_utc()),
            description: None,
            website: Vec::new(),
            timezone: None,
            language: None,
            location: None,
            nums_fans: 0,
            nums_following: 0,
            nums_projects: 0,
            nums_issues: 0,
            nums_comments: 0,
            nums_stars: 0,
            nums_teams: 0,
            nums_groups: 0,
            nums_repositories: 0,
            theme: "system".to_string(),
            avatar: None,
            nums_reviews: 0,
            can_follow: false,
            deleted_at: None,
        };
      
        mapper.insert(model).await
    }
}


impl UsersModel {
    pub fn builder() -> UserBuilder {
        UserBuilder::new()
    }
}