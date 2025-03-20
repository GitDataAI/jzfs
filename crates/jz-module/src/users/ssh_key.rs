use crate::AppModule;
use jz_model::{DeleteOption, ssh_key};
use sea_orm::*;
use serde::Deserialize;
use sha256::Sha256Digest;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SshKeyParam {
    pub name: String,
    pub content: String,
    pub description: Option<String>,
}

impl AppModule {
    pub async fn ssh_key_add(&self, users_uid: Uuid, param: SshKeyParam) -> anyhow::Result<()> {
        let fingerprint = format!(
            "ssh-rsa {} - {} - user - {}",
            param.content.clone(),
            param.name,
            users_uid
        )
        .digest();
        if ssh_key::Entity::find()
            .filter(ssh_key::Column::UserId.eq(users_uid))
            .filter(
                Condition::any()
                    .add(ssh_key::Column::Name.eq(param.name.clone()))
                    .add(ssh_key::Column::Content.eq(param.content.clone())),
            )
            .one(&self.write)
            .await?
            .is_some()
        {
            return Err(anyhow::anyhow!("ssh key or name already exists"));
        }
        let ssh_key = ssh_key::ActiveModel::new(
            users_uid,
            param.name,
            fingerprint,
            param.description,
            param.content,
        );
        ssh_key.insert(&self.write).await?;
        Ok(())
    }
    pub async fn ssh_key_del(
        &self,
        users_uid: Uuid,
        ssh_key_id: Uuid,
    ) -> anyhow::Result<DeleteOption> {
        let active = ssh_key::Entity::delete_by_id(ssh_key_id)
            .filter(ssh_key::Column::UserId.eq(users_uid))
            .exec(&self.write)
            .await?;
        Ok(DeleteOption::from(active))
    }
    pub async fn ssh_key_list(&self, users_uid: Uuid) -> anyhow::Result<Vec<ssh_key::Model>> {
        let ssh_keys = ssh_key::Entity::find()
            .filter(ssh_key::Column::UserId.eq(users_uid))
            .all(&self.read)
            .await?;
        Ok(ssh_keys)
    }
}
