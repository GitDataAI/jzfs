use chrono::Utc;
use uuid::Uuid;
use cert::schema::{result_error_with_msg_data, result_ok_with_data, AppResult};
use issuesd::comment;
use crate::schema::IssueCommentCreateParam;
use crate::service::AppIssueService;
use sea_orm::*;

impl AppIssueService { 
    pub async fn service_publish_comment(&self, param: IssueCommentCreateParam,repo_uid:Uuid,user_uid:Uuid,issue_id:i32)-> AppResult<comment::Model>{
        let Ok(Some(user)) = authd::users::Entity::find_by_id(user_uid).one(&self.db).await else {
            return result_error_with_msg_data("User not found".to_string())
        };
        let issue = match self._issues_get_by_id_and_repo(issue_id,repo_uid).await {
            None => {
                return result_error_with_msg_data("issue not found".to_string());
            }
            Some(x) => {
                x
            }
        };
        let active_model = comment::ActiveModel {
            uid : Set(Uuid::now_v7()),
            author_uid : Set(user.uid),
            content : Set(param.content),
            parent_comment_uid : Set(param.parent_comment_uid),
            is_deleted : Set(false),
            created_at : Set(Utc::now().to_utc()),
            issue_uid: Set(issue.uid),
            updated_at: Set(Utc::now().to_utc()),
        };
        match active_model.insert(&self.db).await {
            Ok(model) => result_ok_with_data(model),
            Err(e) => result_error_with_msg_data(format!("Failed to create comment: {}", e)),
        }
    }
}