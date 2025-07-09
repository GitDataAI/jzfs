use crate::schema::users::{UserAccessTokenCreate, UserAccessTokenDelete, UserAccessTokenItem};
use crate::service::AppWorkHorse;
use authd::security;
use authd::security::SecurityOption;
use cert::schema::{result_error_with_msg_data, result_ok, result_ok_with_data, AppResult, SecurityEventRegisterParam};
use chrono::Utc;
use sea_orm::*;
use sha256::Sha256Digest;
use uuid::Uuid;

impl AppWorkHorse {
    pub async fn service_user_access_key_create(&self, user_uid: Uuid, param: UserAccessTokenCreate, option: SecurityOption) -> AppResult<String>{
        let Ok(Some(user)) = authd::users::Entity::find_by_id(user_uid).one(&self.db).await else {
            return result_error_with_msg_data("User not found".to_string())
        };
        let access_token_key = format!("ack_{}", format!("{}-{}-{}", user_uid, Uuid::now_v7(), Uuid::new_v4()).digest());
        let access_token_fingerprint = format!("{}-{}", user_uid, access_token_key.clone()).digest();
        let access_token = authd::access_key::ActiveModel {
            uid: Set(Uuid::now_v7()),
            title: Set(param.title),
            description: Set(param.description),
            token: Set(access_token_key.clone()),
            use_history: Set(vec![]),
            resource_owner: Set(user.username.clone()),
            resource_owner_uid: Set(user_uid),
            expiration: Set(param.expiration),
            fingerprint: Set(access_token_fingerprint),
            repo_access: Set(param.repo_access),
            email_access: Set(param.email_access),
            event_access: Set(param.event_access),
            follow_access: Set(param.follow_access),
            gpg_access: Set(param.gpg_access),
            ssh_access: Set(param.ssh_access),
            webhook_access: Set(param.webhook_access),
            wiki_access: Set(param.wiki_access),
            project_access: Set(param.project_access),
            issue_access: Set(param.issue_access),
            comment_access: Set(param.comment_access),
            profile_access: Set(param.profile_access),
            created_at: Set(Utc::now().naive_local()),
            updated_at: Set(Utc::now().naive_local()),
        };
        match authd::access_key::Entity::insert(access_token).exec(&self.db).await {
            Ok(_) => {
                let security_event = SecurityEventRegisterParam {
                    title: security::Model::ACCESS_KEY_REGISTER.to_string(),
                    description: None,
                    ip: option.ip,
                    user_agent: option.user_agent,
                    device: option.device,
                    location: option.location,
                    action: "ACCESS KEY REGISTER".to_string(),
                    actor: user.username.clone(),
                    actor_uid: user.uid,// $USER_UID
                    user: user.username,
                    user_uid: user.uid, // $USER_UID
                };
                self.cert.security_event_register(tarpc::context::current(),security_event).await.ok();
                result_ok_with_data(access_token_key)
            }
            Err(e) => {
                result_error_with_msg_data(e.to_string())
            }
        }
    }
    pub async fn service_user_access_token_delete(&self, user_uid: Uuid, param: UserAccessTokenDelete, option: SecurityOption) -> AppResult<()> {
        let Ok(Some(user)) = authd::users::Entity::find_by_id(user_uid).one(&self.db).await else {
            return result_error_with_msg_data("User not found".to_string())
        };
        match authd::access_key::Entity::delete_many()
            .filter(authd::access_key::Column::Uid.eq(param.access_token_uid))
            .filter(authd::access_key::Column::ResourceOwnerUid.eq(user_uid))
            .exec(&self.db)
            .await
        {
            Ok(_) => {
                let security_event = SecurityEventRegisterParam {
                    title: security::Model::ACCESS_KEY_DELETE.to_string(),
                    description: None,
                    ip: option.ip,
                    user_agent: option.user_agent,
                    device: option.device,
                    location: option.location,
                    action: "ACCESS KEY DELETE".to_string(),
                    actor: user.username.clone(),
                    actor_uid: user.uid, // $USER_UID
                    user: user.username,
                    user_uid: user.uid, // $USER_UID
                };
                self.cert.security_event_register(tarpc::context::current(), security_event).await.ok();
                result_ok()
            }
            Err(err) => {
                result_error_with_msg_data(format!("Failed to delete access token: {}", err))
            }
        }
    }
    pub async fn service_user_access_token_list(&self, user_uid: Uuid) -> AppResult<Vec<UserAccessTokenItem>> {
        let Ok(Some(user)) = authd::users::Entity::find_by_id(user_uid).one(&self.db).await else {
            return result_error_with_msg_data("User not found".to_string())
        };
        let Ok(access_tokens) = authd::access_key::Entity::find()
            .filter(authd::access_key::Column::ResourceOwnerUid.eq(user_uid))
            .all(&self.db)
            .await else {
            return result_error_with_msg_data("Failed to get access tokens".to_string())
        };
        let mut access_token_items = Vec::new();
        for access_token in access_tokens { 
            access_token_items.push(crate::schema::users::UserAccessTokenItem {
                uid: access_token.uid,
                title: access_token.title,
                description: access_token.description,
                expiration: access_token.expiration,
                repo_access: access_token.repo_access,
                email_access: access_token.email_access,
                event_access: access_token.event_access,
                follow_access: access_token.follow_access,
                gpg_access: access_token.gpg_access,
                ssh_access: access_token.ssh_access,
                webhook_access: access_token.webhook_access,
                wiki_access: access_token.wiki_access,
                project_access: access_token.project_access,
                issue_access: access_token.issue_access,
                comment_access: access_token.comment_access,
                profile_access: access_token.profile_access,
                created_at: access_token.created_at,
                updated_at: access_token.updated_at,
            })
        };
        result_ok_with_data(access_token_items)
    }
}