use std::collections::HashSet;
use chrono::Utc;
use cert::schema::{result_error_with_msg_data, result_ok_with_data, AppResult};
use issuesd::issues;
use crate::schema::IssueAssigneeUpdateParam;
use crate::service::AppIssueService;
use sea_orm::*;
use uuid::Uuid;

impl AppIssueService {
    pub async fn service_update_issue_assignee(&self, param: IssueAssigneeUpdateParam,new_assignee_uid:Uuid,issue_id: i32, repo_uid: Uuid, user_uid: Uuid,) -> AppResult<()> {
        let Ok(Some(model)) = issues::Entity::find()
            .filter(issues::Column::IssueId.eq(issue_id))
            .filter(issues::Column::RepoUid.eq(repo_uid))
            .one(&self.db)
            .await
        else {
            return result_error_with_msg_data("Issue not found".to_string());
        };
        let mut active = model.clone().into_active_model();
        let mut set = HashSet::new();
        for modelx in model.assignee_uid {
            set.insert(modelx);
        }
        for item in param.update_assignee_uid {
            set.insert(item);
        }
        active.assignee_uid = Set(set.into_iter().collect::<Vec<_>>());
        active.updated_at = Set(Utc::now().naive_local());
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