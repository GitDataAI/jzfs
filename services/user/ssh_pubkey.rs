use sea_orm::{ActiveModelTrait, ColumnTrait};
use std::io;
use sea_orm::{Condition, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::users::ssh;
use crate::services::AppState;

#[derive(Deserialize,Serialize)]
pub struct SSHKeyCreateParma {
    pub name: String,
    pub public_key: String,
    pub description: Option<String>
}



impl AppState {
    pub async fn ssh_key_insert(&self, user_uid: Uuid, params: SSHKeyCreateParma) -> io::Result<()> {
        // check name
        if !ssh::Entity::find()
            .filter(
                Condition::all()
                    .add(ssh::Column::UserId.eq(user_uid))
                    .add(ssh::Column::Name.eq(params.name.clone()))
            )
            .all(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Database Error：{}",e)))?
            .is_empty() {
            return Err(io::Error::new(io::ErrorKind::Other, "Name already exists".to_string()));
        };
        
        let ssh_key = params.public_key.split(" ")
            .map(|x|x.to_string())
            .collect::<Vec<_>>();
        if ssh_key.len() != 3 {
            return Err(io::Error::new(io::ErrorKind::Other, "Public key format error".to_string()));
        };
        if ssh_key[0] != "ssh-rsa" && ssh_key[0] != "ssh-ed25519" {
            return Err(io::Error::new(io::ErrorKind::Other, "Public key proto not support(please you use ssh-rsa or ssh-ed25519)".to_string()));
        };
        if ssh_key[1].len() < 20 {
            return Err(io::Error::new(io::ErrorKind::Other, "Public key format error".to_string()));
        };
        let ssh_key = format!("{} {}", ssh_key[0], ssh_key[1]);
        if !ssh::Entity::find()
            .filter(
                Condition::all()
                    .add(ssh::Column::Content.eq(ssh_key.clone()))
            )
            .all(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Database Error：{}",e)))?
            .is_empty() {
            return Err(io::Error::new(io::ErrorKind::Other, "Public key already exists".to_string()));
        };
        let now = chrono::Utc::now().naive_local();
        ssh::ActiveModel {
            uid: Set(Uuid::new_v4()),
            user_id: Set(user_uid),
            name: Set(params.name),
            content: Set(ssh_key),
            created_at: Set(now),
            description: Set(params.description),
            updated_at: Set(now),
        }.insert(&self.write)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Database Error：{}",e)))?;
        Ok(())
    }
    pub async fn ssh_key_list(&self, user_uid: Uuid) -> io::Result<Vec<ssh::Model>> {
        ssh::Entity::find()
            .filter(ssh::Column::UserId.eq(user_uid))
            .all(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Database Error：{}",e)))
    }
    pub async fn ssh_key_delete(&self, user_uid: Uuid, ssh_uid: Uuid) -> io::Result<()> {
        ssh::Entity::delete_by_id(ssh_uid)
            .filter(ssh::Column::UserId.eq(user_uid))
            .exec(&self.write)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Database Error：{}",e)))?;
        Ok(())
    }
}

