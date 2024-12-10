use sea_orm::*;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::api::dto::users::UserApply;
use crate::metadata::model::users::{users, users_email, users_other};
use crate::metadata::transaction::users::UserTransaction;

impl UserTransaction {
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
                passwd: Set(passwd),
                status: Set(1),
                pro: Set(false),
                theme: Set("default".to_string()),
                localtime: Set("UTC".to_string()),
                timezone: Set("UTC".to_string()),
                company: Set("".to_string()),
                website: Set(vec![]),
                sex: Default::default(),
                description: Default::default(),
                created_at: Set(OffsetDateTime::now_utc()),
                updated_at: Set(OffsetDateTime::now_utc()),
                phone: Default::default(),
                lastlogin: Set(OffsetDateTime::now_utc()),
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
                bind_at: Set(OffsetDateTime::now_utc()),
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
            let result = users_other::ActiveModel{
                uid: Set(Uuid::new_v4()),
                user_id: Set(uid),
                team: Default::default(),
                repo: Default::default(),
                project: Default::default(),
                issue: Default::default(),
                pr: Default::default(),
                commit: Default::default(),
                tag: Default::default(),
                star: Default::default(),
                follow: Default::default(),
                following: Default::default(),
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