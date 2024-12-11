use russh_keys::{PublicKey, PublicKeyBase64};
use sea_orm::*;
use time::{format_description, OffsetDateTime};
use uuid::Uuid;
use crate::api::dto::users::UserKeyList;
use crate::api::service::users::UserService;
use crate::metadata::model::users::users_key;

impl UserService {
    pub async fn get_users_by_pubkey(&self, public_key: &PublicKey) -> anyhow::Result<Uuid>{
        let public_key_base64 = public_key.public_key_base64();
        let key_model = users_key::Entity::find()
            .filter(
                users_key::Column::Pubkey.contains(public_key_base64)
            )
            .one(&self.db)
            .await?;
        match key_model {
            Some(key_model) => Ok(key_model.user_id),
            None => Err(anyhow::anyhow!("Invalid public key"))
        }
    }
    pub async fn list_key(&self, user_id: Uuid) -> anyhow::Result<Vec<UserKeyList>>{
        let key_list = users_key::Entity::find()
            .filter(users_key::Column::UserId.eq(user_id))
            .all(&self.db)
            .await?;
        let mut keys = vec![];
        let format = format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]",
        )?;
        for key in key_list {
            keys.push(UserKeyList{
                created_at: key.created_at.format(&format)?,
                head: key.pubkey[..6].to_string(),
                last_use: key.last_use.format(&format)?,
            })
        }
        Ok(keys)
    }
    pub async fn add_key(&self, user_id: Uuid, name: String, pubkey: String) -> anyhow::Result<()>{
        let arch = users_key::ActiveModel{
            uid: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            pubkey: Set(pubkey),
            name: Set(name),
            created_at: Set(OffsetDateTime::now_utc()),
            last_use: Set(OffsetDateTime::now_utc()),
        };
        arch.insert(&self.db).await?;
        Ok(())
    }
    pub async fn remove_key(&self, user_id: Uuid, uid: Uuid) -> anyhow::Result<()>{
        let arch = users_key::Entity::find_by_id(uid)
            .filter(users_key::Column::UserId.eq(user_id))
            .one(&self.db)
            .await?;
        if arch.is_none(){
            return Err(anyhow::anyhow!("key not found"))
        }
        arch.unwrap().delete(&self.db).await?;
        Ok(())
    }
    
}