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
        
        let Ok(Some(model)) = comment::Entity::find()
            .filter(
                Condition::all()
                    .add(comment::Column::Uid.eq(param.comment_uid))
                    .add(comment::Column::IssueUid.eq(param.issue_id))
                    .add(comment::Column::AuthorUid.eq(author_id))
                    .add(comment::Column::IsDeleted.eq(false))
            )
            .one(&self.db)
            .await else {
            return result_error_with_msg_data("Comment not found".to_string());
        };
        let mut active = model.into_active_model();
        if let Some(update_content) = param.update_content {
            active.content = Set(update_content);
        }
        match active.update(&self.db).await{
            Ok(model) => result_ok_with_data(model),
            Err(err) => result_error_with_msg_data(err.to_string()),
        }
    }
}