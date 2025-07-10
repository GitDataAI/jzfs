use serde::{Deserialize, Serialize};
use uuid::Uuid;
use cert::schema::{result_error_with_msg_data, result_ok_with_data, AppResult};
use crate::service::AppWorkHorse;
use sea_orm::*;
use orgd::org::org_repo;
use userd::user_repo;

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct RepoInitAliasAvailableParam {
    pub owner: Uuid,
    pub org: bool,
    pub name: String,
}

impl AppWorkHorse {
    pub async fn service_repo_init_alias_available(&self, user_uid: Uuid, param: RepoInitAliasAvailableParam) -> AppResult<i32> {
        let Ok(Some(user)) = authd::users::Entity::find_by_id(user_uid).one(&self.db).await else {
            return result_error_with_msg_data("User not found".to_string())
        };
        let repos = if param.org {
            let Ok(repos) = org_repo::Entity::find()
                .filter(org_repo::Column::OrgUid.eq(param.owner))
                .all(&self.db)
                .await else {
                return result_error_with_msg_data("System error".to_string())
            };
            let uids = repos.iter().map(|x|x.uid).collect::<Vec<Uuid>>();
            repository::repository::Entity::find()
                .filter(repository::repository::Column::Uid.is_in(uids))
                .all(&self.db)
                .await
                .unwrap_or(vec![])
        } else {
            let Ok(repos) = user_repo::Entity::find()
                .filter(user_repo::Column::UserUid.eq(param.owner))
                .all(&self.db)
                .await else {
                return result_error_with_msg_data("System error".to_string())
            };
            let uids = repos.iter().map(|x|x.repo_uid).collect::<Vec<Uuid>>();
            repository::repository::Entity::find()
                .filter(repository::repository::Column::Uid.is_in(uids))
                .all(&self.db)
                .await
                .unwrap_or(vec![])
        };
        let count = repos.iter().filter(|x|x.name == param.name).count();
        if count > 0 {
            result_ok_with_data(1)
        } else {
            // TODO Check for banned and system words
            result_ok_with_data(0)
        }
    }
}