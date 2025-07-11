use chrono::Utc;
use crate::schema::IssueSubscribeParam;
use crate::service::AppIssueService;
use cert::schema::{result_error_with_msg, result_ok_with_data, result_ok_with_msg, AppResult};
use issuesd::issue_sub;
use sea_orm::*;
use uuid::Uuid;

impl AppIssueService {
    pub async fn service_issue_unsubscribe(
        &self,
        param: IssueSubscribeParam,
        subscriber_id: Uuid,
        repo_uid: Uuid
    ) -> AppResult<()> {
        let issue = match self._issues_get_by_id_and_repo(param.issue_id, repo_uid).await {
            None => return result_error_with_msg("Issue not found".to_string()),
            Some(issue) => issue,
        };
        
        let delete_result = issue_sub::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(issue_sub::Column::IssueUid.eq(issue.uid))
                    .add(issue_sub::Column::SubscriberUid.eq(subscriber_id))
            )
            .exec(&self.db)
            .await
            .map_err(|e| result_error_with_msg(e.to_string()));
        
        match delete_result { 
            Ok(delete_result) => {
                if delete_result.rows_affected == 0 {
                    return result_error_with_msg("You are not subscribed to this issue".to_string());
                }
                result_ok_with_msg("Successfully unsubscribed".to_string())
            },
            Err(e) => e
        }
    }
}