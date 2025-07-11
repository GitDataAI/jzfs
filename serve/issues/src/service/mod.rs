use cert::rpc::interface::CertInterFaceClient;
use sea_orm::DatabaseConnection;
use session::storage::RedisStorage;
#[derive(Clone)]
pub struct AppIssueService {
    db: DatabaseConnection,
    cache: RedisStorage,
    #[cfg(feature = "distributed")]
    cret: CertInterFaceClient,
    #[cfg(feature = "local")]
    cert: AppCertService
}


pub mod issue_list;
pub mod issues_utils;
pub mod get_issue;
pub mod issues_available_assignee;
pub mod create_issue;
pub mod update_issue;
pub mod create_label;
pub mod update_label;

pub mod delete_label;

pub mod publish_comment;
pub mod update_comment;
pub mod subscribe_issue;
pub mod update_issue_status;
pub mod update_issue_assignee;
pub mod unsubscribe_issue;
pub mod get_issue_subscribers;