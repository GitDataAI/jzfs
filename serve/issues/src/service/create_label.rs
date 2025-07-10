use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;
use cert::schema::{result_error_with_msg_data, result_ok_with_data, AppResult};
use issuesd:: label;
use crate::schema::{IssueLabelCreateParam, LabelCreateParam};
use crate::service::AppIssueService;

impl AppIssueService {
    pub async fn service_create_labels(&self,param:LabelCreateParam, user_uid : Uuid) ->AppResult<label::Model>{
        match self._repo_access_user(param.repo_uid,user_uid).await {
            Ok(bool) => {
                if !bool {
                    return result_error_with_msg_data("You are not allowed to access this repo".to_string());
                }
            }
            Err(_) => {
                return result_error_with_msg_data("Failed to access repo".to_string());
            }
        }
        let active_model = label::ActiveModel {
            label_uid: Set(Uuid::now_v7()),
            repo_uid: Set(param.repo_uid),
            created_at: Set(Utc::now().to_utc()),
            color: Set(param.label_color),
            name: Set(param.label_name),
            description: Set(param.description),
        };
       
        match active_model.insert(&self.db).await {
            Ok(model) => result_ok_with_data(model),
            Err(e) => result_error_with_msg_data(format!("Failed to create issue label: {}", e)),
        }
    }
}
