use serde::Deserialize;
use sha256::Sha256Digest;
use uuid::Uuid;
use jz_model_sqlx::ssh_key::SshKeyList;
use jz_model_sqlx::uuid_v7;
use crate::app::AppService;

#[derive(Deserialize)]
pub struct UsersSshKeyCreateParam {
    pub name: String,
    pub content: String,
    pub description: Option<String>,
}

impl AppService {
    pub async fn users_ssh_key_create(
        &self,
        users_uid: Uuid,
        param: UsersSshKeyCreateParam,
    ) -> anyhow::Result<()> {
        let ssh_key = param.content;
        if ssh_key.len() > 1024 {
            return Err(anyhow::anyhow!("ssh key too long"));
        }
        let split = ssh_key.split(" ").map(|x|x.to_string()).collect::<Vec<String>>();
        if split.len() != 3 {
            return Err(anyhow::anyhow!("ssh key format error"));
        }
        let protoc = split[0].to_string();
        if !protoc.starts_with("ssh-rsa")
            || !protoc.starts_with("rsa-sha2-256")
            || !protoc.starts_with("ssh-ed25519")
            || !protoc.starts_with("rsa-sha2-512")
            || !protoc.starts_with("ecdsa-sha2-nistp256")
            || !protoc.starts_with("ecdsa-sha2-nistp384")
            || !protoc.starts_with("ecdsa-sha2-nistp521")
        {
            return Err(anyhow::anyhow!("Host keys and public key proto not support"));
        }
        let name = if !param.name.is_empty() {
            param.name
        }else {
            split[2].to_string()
        };
        let public_key = split[1].to_string();
        let mapper = self.ssh_key_mapper();
        let query = mapper.query();
        if query.query_by_content(public_key.clone()).await.is_ok() {
            return Err(anyhow::anyhow!("ssh key already exists"));
        }
        let fingerprint = format!("ssh-rsa {} - {} - user - {} - {}", public_key, name, users_uid, uuid_v7()).digest();
        if query.query_by_fingerprint(fingerprint.clone()).await.is_ok() {
            return Err(anyhow::anyhow!("ssh key fingerprint already exists"));
        }
        let builder = jz_model_sqlx::ssh_key::SshKeyBuilder::new()
            .user_id(users_uid)
            .name(name)
            .description(param.description)
            .fingerprint(fingerprint)
            .content(public_key)
            .build();
        mapper.insert(builder).await?;
        Ok(())
    }
    pub async fn users_ssh_key_delete(
        &self,
        users_uid: Uuid,
        ssh_key_id: Uuid,
    ) -> anyhow::Result<()> {
        let mapper = self.ssh_key_mapper();
        let query = mapper.query();
        let ssh_key = query.query_by_uid(ssh_key_id).await?;
        if ssh_key.user_id != users_uid {
            return Err(anyhow::anyhow!("ssh key not found"));
        }
        mapper.relation(ssh_key).delete().await?;
        Ok(())
    }
    pub async fn users_ssh_key_list(&self, users_uid: Uuid) -> anyhow::Result<Vec<SshKeyList>> {
        let mapper = self.ssh_key_mapper();
        let query = mapper.query();
        if let Ok(ssh_keys) = query.query_by_user_id(users_uid).await {
            return Ok(ssh_keys.into_iter().map(|x|x.into()).collect());
        }
        Ok(vec![])
    }
    pub async fn users_ssh_key_find(&self, ssh_key: String) -> anyhow::Result<SshKeyList> {
        let mapper = self.ssh_key_mapper();
        let query = mapper.query();
        if let Ok(ssh_key) = query.query_by_content(ssh_key).await {
            return Ok(ssh_key.into());
        }
        Err(anyhow::anyhow!("ssh key not found"))
    }
}