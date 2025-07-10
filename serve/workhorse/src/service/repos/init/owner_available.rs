use crate::service::AppWorkHorse;
use cert::schema::{result_error_with_msg_data, result_ok_with_data, AppResult};
use orgd::org::{org, org_member};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct RepoInitOwnerAvailableItem {
    pub name: String,
    pub org: bool,
    pub uid: Uuid,
    pub avatar: Option<String>,
}


impl AppWorkHorse {
    pub async fn service_repo_init_owner(&self, user_uid: Uuid) -> AppResult<Vec<RepoInitOwnerAvailableItem>> {
        let Ok(Some(user)) = authd::users::Entity::find_by_id(user_uid).one(&self.db).await else {
            return result_error_with_msg_data("User not found".to_string())
        };
        let mut result = vec![];
        result.push(RepoInitOwnerAvailableItem {
            name: user.username,
            org: false,
            uid: user.uid,
            avatar: user.avatar,
        });
        let members = org_member::Entity::find()
            .filter(org_member::Column::UserUid.eq(user_uid))
            .all(&self.db)
            .await
            .unwrap_or(vec![])
            .iter().filter(|x|x.owner || x.admin)
            .map(|x| x.clone())
            .collect::<Vec<org_member::Model>>();
        for member in members {
            let Ok(Some(org)) = org::Entity::find_by_id(member.org_uid).one(&self.db).await else {
                continue;
            };
            result.push(RepoInitOwnerAvailableItem {
                name: org.name,
                org: true,
                uid: org.uid,
                avatar: org.avatar,
            });
        }
        result_ok_with_data(result)
    }
}