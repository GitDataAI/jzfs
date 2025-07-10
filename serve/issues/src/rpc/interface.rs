use cert::rpc::session::UsersSession;
use cert::schema::AppResult;
use crate::schema::{IssueCreateParam, IssueFetchParam, IssueCommentUpdateParam,
                    IssueLabelDeleteParam, IssueUpdateParam, IssueCommentCreateParam,
                    IssueSubscribeParam, IssueStatusUpdateParam, IssueAssigneeUpdateParam,
                    IssueLabelUnlinkParam, IssueLabelLinkParam};
use issuesd::{ issues,label,comment,issue_labels,issue_sub } ;

#[tarpc::service]
pub trait IssueInterFace {
    async fn get_issue(param: IssueFetchParam)-> AppResult<issues::Model>;
    async fn get_issue_list(param: IssueFetchParam)-> AppResult<serde_json::Value>;
    async fn create_issue(param:IssueCreateParam)-> AppResult<issues::Model>;
    async fn update_issue(param:IssueUpdateParam)-> AppResult<issues::Model>;
    
    async fn create_label(param:IssueCreateParam)-> AppResult<label::Model>;
    async fn update_label(param:IssueUpdateParam)-> AppResult<label::Model>;
    async fn link_issue_label(param:IssueLabelLinkParam)-> AppResult<issue_labels::Model>;
    async fn unlink_issue_label(param:IssueLabelUnlinkParam)-> AppResult<issue_labels::Model>;
    async fn delete_label(parm:IssueLabelDeleteParam)-> AppResult<()>;    
    async fn publish_comment(param:IssueCommentCreateParam)-> AppResult<comment::Model>;
    async fn update_comment(parma:IssueCommentUpdateParam)-> AppResult<comment::Model>;
    async fn subscribe_issue(parma:IssueSubscribeParam)-> AppResult<()>;
    async fn unsubscribe_issue(parma:IssueSubscribeParam)-> AppResult<()>;
    async fn get_issue_subscribers(param:IssueFetchParam)-> AppResult<Vec<UsersSession>>;
    async fn update_issue_status(param:IssueStatusUpdateParam)-> AppResult<issue_sub::Model>;
    async fn update_issue_assignee(parma:IssueAssigneeUpdateParam)-> AppResult<issue_sub::Model>;
}
