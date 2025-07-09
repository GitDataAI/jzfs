use anyhow::anyhow;
use crate::service::AppIssueService;
use issuesd::issues;
use sea_orm::*;
use uuid::Uuid;
use userd::user_repo;

impl AppIssueService {
    pub(crate) async fn _issues_get_by_id_and_repo(&self, issue_id: i32, repo_uid: Uuid) -> Option<issues::Model> {
        match issues::Entity::find()
            .filter(
                Condition::all()
                    .add(issues::Column::IssueId.eq(issue_id))
                    .add(issues::Column::RepoUid.eq(repo_uid))
            )
            .one(&self.db)
            .await {
            Ok(result) => {
                result
            }
            _ => None
        }
    } 
    pub(crate) async fn _repo_access_user(&self, repo_uid: Uuid, user_uid: Uuid) -> anyhow::Result<bool> {
        let result = match user_repo::Entity::find()
            .filter(
                Condition::all()
                    .add(user_repo::Column::RepoUid.eq(repo_uid))
                    .add(user_repo::Column::UserUid.eq(user_uid))
            )
            .one(&self.db)
            .await? {
            Some(_) => true,
            None => false,
        };
        if !result {
            // TODO: 增加用户在组织中，该仓库属于组织
        }
        Ok(result)
        
    }
}