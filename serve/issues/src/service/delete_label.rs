use uuid::Uuid;
use cert::schema::{result_error_with_msg_data, result_ok_with_data, AppResult};
use issuesd::label;
use crate::schema::IssueLabelDeleteParam;
use crate::service::AppIssueService;
use sea_orm::*;

impl AppIssueService {
    pub async fn service_delete_label(&self, param: IssueLabelDeleteParam,user_id : Uuid,repo_uid:Uuid) -> AppResult<()> {
        match self._repo_access_user(repo_uid,user_id).await {
            Ok(bool) => {
                if !bool {
                    return result_error_with_msg_data("You are not allowed to access this repo".to_string());
                }
            }
            Err(_) => {
                return result_error_with_msg_data("Failed to access repo".to_string())
            }
        }
        match label::Entity::delete_many()
            .filter(label::Column::RepoUid.eq(repo_uid))
            .filter(label::Column::LabelUid.eq(param.label_id))
            .exec(&self.db)
            .await
        {
            Ok(_) => result_ok_with_data(()),
            Err(e) => result_error_with_msg_data(e.to_string())
        }
    }
}