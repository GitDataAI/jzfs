use crate::schema::IssueCreateParam;
use crate::service::AppIssueService;
use cert::schema::{result_error_with_msg_data, result_ok_with_data, AppResult};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use issuesd::issues;

impl AppIssueService {
    pub async fn service_create_issue(&self, param: IssueCreateParam,author_uid:Uuid, repo_uid:Uuid) -> AppResult<issues::Model> {
        let active_model = issues::ActiveModel {
            uid : Set(Uuid::now_v7()),
            title : Set(param.title),
            description : Set(param.description),
            created_at: Set(Utc::now().naive_local()),
            updated_at : Set(Utc::now().naive_local()),
            issue_id : Set(0),
            repo_uid : Set(repo_uid),
            author_uid : Set(author_uid),
            assignee_uid : Set(param.assignee_uid),
            status : Set("open".to_string()),
            priority_label_uid : Set(None),
            is_deleted : Set(false),
        };
        match active_model.insert(&self.db).await {
            Ok(model) => result_ok_with_data(model),
            Err(e) => result_error_with_msg_data(format!("Failed to create issue: {}", e)),
        }
    }
}
