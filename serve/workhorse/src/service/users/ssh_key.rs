use chrono::Utc;
use sea_orm::*;
use sha256::Sha256Digest;
use uuid::Uuid;
use authd::{security, sshkey};
use cert::schema::{result_error_with_msg, result_error_with_msg_data, result_ok, result_ok_with_data, AppResult, SecurityEventRegisterParam};
use crate::schema::users;
use crate::schema::users::UserSshKeyDelete;
use crate::service::AppWorkHorse;

impl AppWorkHorse {
    pub async fn service_ssh_key_create(&self, user_uid: Uuid, param: users::UserSshKeyCreate, option: security::SecurityOption) -> AppResult<()> {
        let fingerprint = format!("SHA256:{}", format!("{}-{}-{}", user_uid, param.name,param.content).digest());
        let content = param.content.clone().split(" ").into_iter().map(|x|x.to_string()).collect::<Vec<String>>();
        let content = if content.len() == 3 {
            content[1].to_string()
        } else if content.len() == 2 {
            content[1].to_string()
        } else if content.len() == 1 {
            content[0].to_string()
        } else {
            return result_error_with_msg("ssh key content error".to_string());
        };
        let Ok(Some(user)) = authd::users::Entity::find_by_id(user_uid)
            .one(&self.db)
            .await else { 
            return result_error_with_msg("user not found".to_string());
        };
        let ssh_key = sshkey::ActiveModel {
            uid: Set(Uuid::now_v7()),
            user_id: Set(user_uid),
            name: Set(param.name),
            fingerprint: Set(fingerprint),
            description: Set(param.description),
            content: Set(content),
            created_at: Set(Utc::now().naive_local()),
            updated_at: Set(Utc::now().naive_local()),
        };
        match sshkey::Entity::insert(ssh_key).exec(&self.db).await {
            Ok(_) => {
                let security_event = SecurityEventRegisterParam {
                    title: security::Model::SSHKEY_REGISTER.to_string(),
                    description: None,
                    ip: option.ip,
                    user_agent: option.user_agent,
                    device: option.device,
                    location: option.location,
                    action: "SSH KEY REGISTER".to_string(),
                    actor: user.username.clone(),
                    actor_uid: user.uid,// $USER_UID
                    user: user.username,
                    user_uid: user.uid, // $USER_UID
                };
                self.cert.security_event_register(tarpc::context::current(),security_event).await.ok();
                result_ok()
            }
            Err(error) => {
                result_error_with_msg(error.to_string())
            }
        }
    }
    pub async fn service_user_ssh_key_delete(&self,user_uid: Uuid, param: UserSshKeyDelete, option: security::SecurityOption) -> AppResult<()> {
        let Ok(Some(user)) = authd::users::Entity::find_by_id(user_uid).one(&self.db).await else { 
            return result_error_with_msg("User not found".to_string())
        };
        match sshkey::Entity::delete_many()
            .filter(sshkey::Column::Uid.eq(param.ssh_key_uid))
            .filter(sshkey::Column::UserId.eq(user_uid))
            .exec(&self.db)
            .await {
            Ok(_) => {
                let security_event = SecurityEventRegisterParam {
                    title: security::Model::SSHKEY_REGISTER.to_string(),
                    description: None,
                    ip: option.ip,
                    user_agent: option.user_agent,
                    device: option.device,
                    location: option.location,
                    action: "SSH KEY REGISTER".to_string(),
                    actor: user.username.clone(),
                    actor_uid: user.uid,// $USER_UID
                    user: user.username,
                    user_uid: user.uid, // $USER_UID
                };
                self.cert.security_event_register(tarpc::context::current(),security_event).await.ok();
                result_ok()
            }
            Err(error) => {
                result_error_with_msg(error.to_string())
            }
            }
    }
    pub async fn service_user_ssh_key_list(&self, user_id: Uuid) -> AppResult<Vec<sshkey::Model>> {
        let ssh_key = sshkey::Entity::find()
            .filter(sshkey::Column::UserId.eq(user_id))
            .all(&self.db)
            .await;
        match ssh_key {
            Ok(mut result) => {
                result = result.iter().map(| x|{
                    let mut d = x.clone();
                    d.content = "".to_string();
                    d
                }).collect::<Vec<sshkey::Model>>();
                result_ok_with_data(result)
            }
            Err(err) => {
                result_error_with_msg_data(err.to_string())
            }
        }
    }
}