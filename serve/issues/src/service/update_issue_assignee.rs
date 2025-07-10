use cert::schema::{result_error_with_msg_data, result_ok_with_data, AppResult};
use issuesd::issues;
use crate::schema::IssueAssigneeUpdateParam;
use crate::service::AppIssueService;
use sea_orm::*;
use uuid::Uuid;

impl AppIssueService {
    pub async fn service_update_issue_assignee(&self, param: IssueAssigneeUpdateParam,new_assignee_uid:Uuid) -> AppResult<()> {
        let Ok(Some(model)) = issues::Entity::find()
            .filter(issues::Column::IssueId.eq(param.issue_id))
            .filter(issues::Column::RepoUid.eq(param.repo_uid))
            .one(&self.db)
            .await
        else {
            return result_error_with_msg_data("Issue not found".to_string());
        };
        let mut active = model.into_active_model();
        active.assignee_uid = Set(Some(vec![new_assignee_uid]));
        match active.update(&self.db).await {
            Ok(_) => {
                result_ok_with_data(())
            }
            Err(err) => {
                result_error_with_msg_data(err.to_string())
            }
        }
    }
}