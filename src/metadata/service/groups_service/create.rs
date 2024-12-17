use sea_orm::*;
use sha256::Sha256Digest;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::api::dto::groups_dto::GroupCreate;
use crate::metadata::model::groups::{groups, groups_users};
use crate::metadata::service::groups_service::GroupService;

impl GroupService {
    pub async fn create(&self, dto: GroupCreate, ops_id: Uuid) -> anyhow::Result<()>{
        let txn = self.db.begin().await.unwrap();
        let uid = Uuid::new_v4();
        let result = groups::ActiveModel{
            uid: Set(uid),
            name: Set(dto.name.clone()),
            username: Set(dto.name),
            passwd: Set("".digest()),
            status: Set(1),
            pro: Set(false),
            theme: Set("default".to_string()),
            localtime: Set("UTC".to_string()),
            timezone: Set("UTC".to_string()),
            company: Set("".to_string()),
            website: Set(vec![]),
            description: Default::default(),
            created_at: Set(OffsetDateTime::now_utc()),
            updated_at: Set(OffsetDateTime::now_utc()),
            phone: Default::default(),
            lastlogin: Set(OffsetDateTime::now_utc()),
            avatar: Default::default(),
            is_groups: Set(true),
        }
            .insert(&txn)
            .await;
        match result{
            Ok(_) => {},
            Err(e) => {
                txn.rollback().await?;
                return Err(anyhow::anyhow!(e))
            }
        }
        let result = groups_users::ActiveModel{
            uid: Default::default(),
            group_id: Set(uid),
            users_id: Set(ops_id),
            access: Set(3),
            join_at: Set(OffsetDateTime::now_utc()),
        }
            .insert(&txn)
            .await;
        match result{
            Ok(_) => {},
            Err(e) => {
                txn.rollback().await?;
                return Err(anyhow::anyhow!(e))
            }
        }
        txn.commit().await?;
        Ok(())
    }
}