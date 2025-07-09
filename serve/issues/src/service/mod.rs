use sea_orm::DatabaseConnection;
use tarpc::context::Context;
use cert::rpc::interface::CertInterFaceClient;
use cert::rpc::session::UsersSession;
use issuesd::issues;
use issuesd::issues::Model;
use session::storage::RedisStorage;
use crate::rpc::interface::IssueInterFace;
use crate::schema::{ IssueAssigneeUpdateParam, IssueCommentCreateParam, IssueCommentUpdateParam, IssueCreateParam, IssueFetchParam, IssueLabelDeleteParam, IssueLabelLinkParam, IssueLabelUnlinkParam, IssueStatusUpdateParam, IssueSubscribeParam, IssueUpdateParam};
use cert::schema::AppResult;
#[derive(Clone)]
pub struct AppIssueService {
    db: DatabaseConnection,
    cache: RedisStorage,
    #[cfg(feature = "distributed")]
    cret: CertInterFaceClient,
    #[cfg(feature = "local")]
    cert: AppCertService
}

impl IssueInterFace for AppIssueService {
    async fn get_issue(self,context:Context,param: IssueFetchParam) -> AppResult<Model> {
        self.service_get_issue(param).await
    }
    async fn get_issue_list(self,_context: Context, param: IssueFetchParam) -> AppResult<serde_json::Value> {
        self.service_get_issue_list(param).await
    }
    async fn create_issue(self, context: Context,param: IssueCreateParam) -> AppResult<issues::Model> {
        self.service_create_issue(param).await
    }

    async fn update_issue(self, context: Context, param: IssueUpdateParam) -> AppResult<Model> {
      todo!()
    }

    async fn create_label(self, context: Context, param: IssueCreateParam) -> AppResult<issuesd::label::Model> {
        todo!()
    }

    async fn update_label(self, context: Context, param: IssueUpdateParam) -> AppResult<issuesd::label::Model> {
        todo!()
    }

    async fn link_issue_label(self, context: Context, param: IssueLabelLinkParam) -> AppResult<issuesd::issue_labels::Model> {
        todo!()
    }

    async fn unlink_issue_label(self, context: Context, param: IssueLabelUnlinkParam) -> AppResult<issuesd::issue_labels::Model> {
        todo!()
    }

    async fn delete_label(self, context: Context, parm: IssueLabelDeleteParam) -> AppResult<()> {
        todo!()
    }

    async fn publish_comment(self, context: Context, param: IssueCommentCreateParam) -> AppResult<issuesd::comment::Model> {
        todo!()
    }

    async fn update_comment(self, context: Context, parma: IssueCommentUpdateParam) -> AppResult<issuesd::comment::Model> {
        todo!()
    }

    async fn subscribe_issue(self, context: Context, parma: IssueSubscribeParam) -> AppResult<()> {
        todo!()
    }

    async fn unsubscribe_issue(self, context: Context, parma: IssueSubscribeParam) -> AppResult<()> {
        todo!()
    }

    async fn get_issue_subscribers(self, context: Context, param: IssueFetchParam) -> AppResult<Vec<UsersSession>> {
        todo!()
    }

    async fn update_issue_status(self, context: Context, param: IssueStatusUpdateParam) -> AppResult<issuesd::issue_sub::Model> {
        todo!()
    }

    async fn update_issue_assignee(self, context: Context, parma: IssueAssigneeUpdateParam) -> AppResult<issuesd::issue_sub::Model> {
        todo!()
    }
}

mod issue_list;
pub mod issues_utils;
mod get_issue;
mod create_issue;
mod update_issue;