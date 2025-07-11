use chrono::Utc;
use crate::schema::IssueSubscribeParam;
use crate::service::AppIssueService;
use cert::schema::{result_error_with_msg, result_error_with_msg_data, result_ok_with_msg, AppResult};
use issuesd::issue_sub;
use sea_orm::*;
use uuid::Uuid;

impl AppIssueService {
    pub async fn service_issue_sub(
        &self,
        param: IssueSubscribeParam,
        subscriber_id: Uuid, 
        repo_uid: Uuid
    ) -> AppResult<()> {
        let issue = match self._issues_get_by_id_and_repo(param.issue_id, repo_uid).await {
            None => {
                return result_error_with_msg_data("Issue not found".to_string());
            }
            Some(issue) => issue,
        };

        let new_sub = issue_sub::ActiveModel {
            uid: Set(Uuid::now_v7()),
            subscriber_uid: Set(subscriber_id),
            issue_uid: Set(issue.uid),
            created_at: Set(Utc::now().naive_local())
        };
        
        match  issue_sub::Entity::insert(new_sub)
            .exec(&self.db)
            .await
            .map_err(|e| result_error_with_msg(e.to_string())) { 
            Ok(_) => result_ok_with_msg("Successfully subscribed".to_string()),
            Err(e) => e
        }
    }
    
}
