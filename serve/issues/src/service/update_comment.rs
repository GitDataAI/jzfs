use uuid::Uuid;
use cert::schema::{result_error_with_msg_data, result_ok_with_data, AppResult};
use issuesd::comment;
use crate::schema::IssueCommentUpdateParam;
use crate::service::AppIssueService;
use sea_orm::*;
use issuesd::comment::Model;

impl AppIssueService {
    pub async fn service_update_comment(&self, param: IssueCommentUpdateParam,repo_uid: Uuid,author_id : Uuid) -> AppResult<Model> {
        match self._repo_access_user(repo_uid,author_id).await {
            Ok(bool) => {
                if !bool {
                    return result_error_with_msg_data("You are not allowed to access this repo".to_string());
                }
            }
            Err(_) => {
                return result_error_with_msg_data("Failed to access repo".to_string());
            }
        }
        let mut query = comment::Entity::find();
        let Ok(Some(model)) = query
            .filter(comment::Column::IssueUid.eq(param.issue_id))
            .one(&self.db)
            .await
        else {
            return result_error_with_msg_data("Issue not found".to_string());
        };
        let mut active = model.into_active_model();
        if param.update_content.is_empty() {
            return result_error_with_msg_data("Content cannot be empty".to_string());
        }
        active.content = Set(param.update_content);
        match active.update(&self.db).await{
            Ok(model) => result_ok_with_data(model),
            Err(err) => result_error_with_msg_data(err.to_string()),
        }
    }
}