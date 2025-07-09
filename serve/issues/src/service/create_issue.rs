use crate::schema::IssueCreateParam;
use crate::service::AppIssueService;
use cert::schema::{result_error_with_msg_data, result_ok_with_data, AppResult};
use chrono::Utc;
use issuesd::issues::{self, ActiveModel as IssueActiveModel, Entity as IssueEntity};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

impl AppIssueService {
    pub async fn service_create_issue(&self, param: IssueCreateParam) -> AppResult<issues::Model> {
        let db: &DatabaseConnection = &self.db;

        let active_model = IssueActiveModel {
            uid : Set(Uuid::now_v7()),
            title : Set(param.title),
            description : Set(param.description),
            created_at: Set(Utc::now().naive_local()),
            updated_at : Set(Utc::now().naive_local()),
            issue_id : Set(0),
            repo_uid : Set(param.repo_uid),
            author_uid : Set(param.author_uid),
            assignee_uid : Set(param.assignee_uid),
            state : Set("open".to_string()),
            priority_label_uid : Set(None),
            is_deleted : Set(false),
        };

        match IssueEntity::insert(active_model).exec_with_returning(db).await {
            Ok(model) => result_ok_with_data(model),
            Err(e) => result_error_with_msg_data(format!("Failed to create issue: {}", e)),
        }
    }
}
