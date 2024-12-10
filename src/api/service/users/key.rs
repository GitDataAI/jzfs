use russh_keys::{PublicKey, PublicKeyBase64};
use sea_orm::*;
use uuid::Uuid;
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
}