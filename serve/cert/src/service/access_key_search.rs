use authd::{access_key, users};
use crate::schema::{AccessKeySearch, AppResult};
use crate::service::AppCertService;
use sea_orm::*;

impl AppCertService {
    pub async fn service_access_key_search(
        &self,
        param: AccessKeySearch,
    ) -> AppResult<users::Model> {
        let model = access_key::Entity::find()
            .filter(access_key::Column::Token.eq(param.access_key))
            .one(&self.db)
            .await;
        let Ok(Some(model)) = model else {
            return AppResult {
                code: 400,
                data: None,
                msg: None,
            };
        };
        if param.req_comment_access <= model.comment_access
            && param.req_email_access <= model.email_access
            && param.req_event_access <= model.event_access
            && param.req_follow_access <= model.follow_access
            && param.req_gpg_access <= model.gpg_access
            && param.req_issue_access <= model.issue_access
            && param.req_profile_access <= model.profile_access
            && param.req_project_access <= model.project_access
            && param.req_repo_access <= model.repo_access
            && param.req_webhook_access <= model.webhook_access
            && param.req_wiki_access <= model.wiki_access
            && param.req_ssh_access <= model.ssh_access
        {
            let user = users::Entity::find()
                .filter(users::Column::Uid.eq(model.resource_owner_uid))
                .one(&self.db)
                .await;
            match user {
                Ok(u) => AppResult {
                    code: 200,
                    data: u,
                    msg: None,
                },
                Err(error) => AppResult {
                    code: 500,
                    data: None,
                    msg: Some(error.to_string()),
                },
            }
        } else {
            AppResult {
                code: 401,
                data: None,
                msg: Some("AccessKey not found".to_string()),
            }
        }
    }
}
