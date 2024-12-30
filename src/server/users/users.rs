use crate::error::{JZError, JZResult};
use crate::models::users::{email, users};
use crate::server::MetaData;
use anyhow::anyhow;
use sea_orm::*;
use uuid::Uuid;

impl MetaData {
    pub async fn users_login(&self, username: String, password: String) -> JZResult<users::Model> {
        let models = users::Entity::find()
            .filter(users::Column::Username.eq(username.clone()))
            .one(&self.database)
            .await?;
        if models.is_none() {
            let models = users::Entity::find()
                .filter(users::Column::MainEmail.eq(username))
                .one(&self.database)
                .await?;
            if models.is_none() {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[02]UserName or Password Err"
                )));
            }
            if models.clone().unwrap().password != sha256::digest(password) {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[021]UserName or Password Err"
                )));
            }
            let models = models.unwrap();
            let mut arch = models.clone().into_active_model();
            arch.hasused = Set(chrono::Local::now().timestamp());
            arch.update(&self.database).await?;
            return Ok(models);
        }
        if models.clone().unwrap().password != sha256::digest(password.clone()) {
            return Err(JZError::Other(anyhow::anyhow!(
                "[01]UserName or Password Err"
            )));
        }
        let models = models.unwrap();
        let mut arch = models.clone().into_active_model();
        arch.hasused = Set(chrono::Local::now().timestamp());
        arch.update(&self.database).await?;
        Ok(models)
    }
    pub async fn users_apply(
        &self,
        username: String,
        password: String,
        email: String,
    ) -> JZResult<Uuid> {
        if self.check_user_username(username.clone()).await? {
            return Err(JZError::Other(anyhow!("[03] Username already exists")));
        }
        if self.check_user_email(email.clone()).await? {
            return Err(JZError::Other(anyhow!("[04] Email already exists")));
        }
        let txn = self.database.begin().await?;
        let user_id = Uuid::new_v4();
        let result = users::ActiveModel {
            uid: Set(user_id),
            name: Set(username.clone()),
            username: Set(username),
            bio: Set(None),
            pronouns: Set(None),
            company: Set(None),
            location: Set(None),
            localtime: Set(None),
            i18n: Set(None),
            website: Set(vec![]),
            orcid: Set(None),
            social: Set(vec![]),
            theme: Set(String::from("Light")),
            pinned: Set(vec![]),
            followers: Set(0),
            following: Set(0),
            repository: Set(0),
            stars: Set(0),
            watching: Set(0),
            package: Set(0),
            release: Set(0),
            password: Set(sha256::digest(password)),
            main_email: Set(email.clone()),
            visible_email: Set(true),
            created: Set(chrono::Local::now().timestamp()),
            updated: Set(chrono::Local::now().timestamp()),
            mentioned: Set(true),
            pro: Set(false),
            avatar_url: Set(None),
            hasused: Set(chrono::Local::now().timestamp()),
        }
        .insert(&txn)
        .await;
        match result {
            Ok(_) => {}
            Err(err) => {
                txn.rollback().await?;
                return Err(JZError::Other(anyhow!("[05] {:?}", err)));
            }
        };
        let result = email::ActiveModel {
            uid: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            content: Set(email),
            main: Set(true),
            primary: Set(true),
            created: Set(chrono::Local::now().timestamp()),
            updated: Set(chrono::Local::now().timestamp()),
            hasused: Set(chrono::Local::now().timestamp()),
        }
        .insert(&txn)
        .await;
        match result {
            Ok(_) => {}
            Err(err) => {
                txn.rollback().await?;
                return Err(JZError::Other(anyhow!("[06] {:?}", err)));
            }
        };
        txn.commit().await?;
        Ok(user_id)
    }
    pub async fn users_search(
        &self,
        keyword: String,
        page: u64,
        size: u64,
    ) -> JZResult<Vec<users::Model>> {
        let models = users::Entity::find()
            .filter(
                Condition::any()
                    .add(users::Column::Username.contains(keyword.clone()))
                    .add(users::Column::Name.contains(keyword.clone()))
                    .add(users::Column::Bio.contains(keyword.clone()))
                    .add(users::Column::Company.contains(keyword.clone()))
                    .add(users::Column::Website.contains(keyword.clone()))
                    .add(users::Column::Social.contains(keyword.clone())),
            )
            .offset(page * size)
            .limit(size)
            .all(&self.database)
            .await?;
        Ok(models)
    }
    pub async fn users_info_username(&self, username: String) -> JZResult<users::Model> {
        let models = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.database)
            .await?;
        if models.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[07] User Not Found")));
        }
        Ok(models.unwrap())
    }
    pub async fn users_info_uid(&self, uid: Uuid) -> JZResult<users::Model> {
        let models = users::Entity::find_by_id(uid).one(&self.database).await?;
        if models.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[08] User Not Found")));
        }
        Ok(models.unwrap())
    }
}
