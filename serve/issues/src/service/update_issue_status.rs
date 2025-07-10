use uuid::Uuid;
use cert::schema::{result_error_with_msg_data, result_ok_with_data, AppResult};
use issuesd::issues;
use crate::schema::IssueStatusUpdateParam;
use crate::service::AppIssueService;
use sea_orm::*;

impl AppIssueService { 
    pub async fn service_update_issue_status(&self, param: IssueStatusUpdateParam, user_uid: Uuid)-> AppResult<()>{
        let Ok(Some(model)) = issues::Entity::find()
            .filter(issues::Column::IssueId.eq(param.issue_id))
            .filter(issues::Column::RepoUid.eq(param.repo_uid))
            .one(&self.db)
            .await
        else {
            return result_error_with_msg_data("issue not found".to_string());
        };
        let mut active = model.into_active_model();
        active.status = Set(param.status);
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