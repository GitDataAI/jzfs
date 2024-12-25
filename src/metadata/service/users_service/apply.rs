use crate::metadata::service::users_service::UserService;
use sea_orm::*;
use sha256::Sha256Digest;
use time::{format_description, OffsetDateTime};
use uuid::Uuid;
use crate::api::dto::user_dto::UserApply;
use crate::metadata::model::users::{users, users_data, users_email};

impl UserService {
    pub async fn apply(&self, dto: UserApply) -> anyhow::Result<Uuid>{
        let txn = self.db.begin().await?;
        let username = dto.username;
        let email = dto.email;
        let passwd = dto.password;
        let uid = Uuid::new_v4();
        {
            let result = users::ActiveModel{
                uid: Set(uid),
                name: Set(username.clone()),
                username: Set(username),
                passwd: Set(passwd.digest()),
                status: Set(1),
                pro: Set(false),
                theme: Set("default".to_string()),
                localtime: Set("UTC".to_string()),
                timezone: Set("UTC".to_string()),
                company: Set("".to_string()),
                website: Set(vec![]),
                description: Set(None),
                created_at: Set(OffsetDateTime::now_utc()),
                updated_at: Set(OffsetDateTime::now_utc()),
                phone: Set(Option::from(None)),
                lastlogin: Set(OffsetDateTime::now_utc()),
                avatar: Default::default(),
                is_groups: Set(false),
            }
                .insert(&txn)
                .await;
            match result{
                Ok(_) => {

                },
                Err(e) =>{
                    txn.rollback().await?;
                    return Err(anyhow::anyhow!(e))
                }
            }
            let result = users_email::ActiveModel{
                uid: Set(Uuid::new_v4()),
                user_id: Set(uid),
                name: Set("default".to_string()),
                email: Set(email),
                is_public: Set(true),
                verified: Set(true),
                bind_at: Set(OffsetDateTime::now_utc().format(&format_description::parse(
                    "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]",
                )?)?),
                main: Set(true),
            }
                .insert(&txn)
                .await;
            match result{
                Ok(_) => {

                },
                Err(e) => {
                    txn.rollback().await?;
                    return Err(anyhow::anyhow!(e))
                }
            }
            let result = users_data::ActiveModel{
                uid: Set(Uuid::new_v4()),
                user_id: Set(uid),
                repo: Set(vec![]),
                project:Set(vec![]),
                issue: Set(vec![]),
                pr: Set(vec![]),
                commit: Set(vec![]),
                tag: Set(vec![]),
                star: Set(vec![]),
                follow: Set(vec![]),
                following: Set(vec![]),
                watcher: Set(vec![]),
            }
                .insert(&txn)
                .await;
            match result{
                Ok(_) => {

                },
                Err(e) => {
                    txn.rollback().await?;
                    return Err(anyhow::anyhow!(e))
                }
            }
        }
        txn.commit().await?;
        Ok(uid.clone())
    }

}