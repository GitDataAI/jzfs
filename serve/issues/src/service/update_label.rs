use crate::service::AppIssueService;
use cert::schema::{result_error_with_msg_data, result_ok_with_data, AppResult};
use issuesd::label;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct LabelUpdateParam {
    pub repo_uid: Uuid,
    pub label_uid: Uuid,
    pub new_name: Option<String>,
    pub new_description: Option<String>,
    pub new_color: Option<String>,
}

impl AppIssueService {
    pub async fn service_update_label(&self, param: LabelUpdateParam,user_id : Uuid) -> AppResult<label::Model> {
        match self._repo_access_user(param.repo_uid,user_id).await {
            Ok(bool) => {
                if !bool {
                    return result_error_with_msg_data("You are not allowed to access this repo".to_string());
                }
            }
            Err(_) => {
                return result_error_with_msg_data("Failed to access repo".to_string());
            }
        }
        let Ok(Some(model)) = label::Entity::find()
            .filter(label::Column::RepoUid.eq(param.repo_uid))
            .filter(label::Column::LabelUid.eq(param.label_uid))
            .one(&self.db)
            .await
        else {
            return result_error_with_msg_data("Label not found".to_string());
        };
        let mut active = model.into_active_model();
        if let Some(title) = param.new_name {
            active.name = Set(title);
        }
        if let Some(description) = param.new_description {
            active.description = Set(Some(description));
        }
        if let Some(color) = param.new_color {
            active.color = Set(color);
        }
        match active.update(&self.db).await {
            Ok(model) => result_ok_with_data(model),
            Err(err) => result_error_with_msg_data(err.to_string()),
        }
    }
}