use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;
use cert::schema::{result_error_with_msg, result_error_with_msg_data, result_ok_with_data, AppResult};
use issues::Model;
use issuesd::issues;
use crate::schema::IssueUpdateParam;
use crate::service::AppIssueService;

impl AppIssueService {
    pub async fn service_update_issue(&self, param: IssueUpdateParam,issue_id: i32, repo_uid: Uuid, author_uid: Uuid,) -> AppResult<Model> {
        let Ok(Some(model)) = issues::Entity::find()
            .filter(issues::Column::IssueId.eq(issue_id))
            .filter(issues::Column::RepoUid.eq(repo_uid))
            // TODO: check permission
            .filter(issues::Column::AuthorUid.eq(author_uid))
            .one(&self.db)
            .await
        else { 
            return result_error_with_msg_data("Issue not found".to_string());
        };
        let mut active = model.into_active_model();
        if let Some(title) = param.new_title {
            active.title = Set(title);
        }
        if let Some(description) = param.new_description {
            active.description = Set(Some(description));
        }
       match active.update(&self.db).await{ 
           Ok(model) => result_ok_with_data(model),
           Err(err) => result_error_with_msg_data(err.to_string()),
       }

    }
}
