use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueFetchParam {
    pub issue_id: Uuid,
    pub repo_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueUpdateParam {
    pub issue_id: Uuid,
    pub repo_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueCreateParam {
    pub repo_uid: Uuid,
    pub content: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueCommentFetchParam {
    pub issue_id: Uuid,
    pub repo_uid: Uuid,
    pub comment_id: Uuid,
    
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueCommentUpdateParam {
    pub issue_id: Uuid,
    pub repo_uid: Uuid,
    pub update_comment_id: Uuid,
    pub update_content: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueCommentCreateParam {
    pub issue_id: Uuid,
    pub content: String,
    pub comment_uid: Option<Uuid>,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueLabelFetchParam {
    pub issue_id: Uuid,
    pub repo_uid: Uuid,
    pub label_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueLabelUpdateParam {
    pub issue_uid: Uuid,
    pub repo_uid: Uuid,
    pub update_label_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueLabelCreateParam {
    pub issue_uid: Uuid,
    pub repo_uid: Uuid,
    pub label_name: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueLabelLinkParam {
    pub issue_uid: Uuid,
    pub label_uid: Uuid,
    pub repo_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueLabelUnlinkParam {
    pub issue_uid: Uuid,
    pub label_uid: Uuid,
    pub repo_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueLabelDeleteParam {
    pub label_uid: Uuid,
    pub repo_uid: Uuid,
    pub delete_label_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueSubscribeParam {
    pub issue_uid: Uuid,
    pub repo_uid: Uuid,
    pub subscribe_uid: Uuid,
    pub is_subscribed: bool,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueStatusUpdateParam {
    pub issue_uid: Uuid,
    pub repo_uid: Uuid,
    pub status: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueAssigneeUpdateParam {
    pub issue_uid: Uuid,
    pub repo_uid: Uuid,
    pub update_assignee_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueHistoryFetchParam {
    pub issue_uid: Uuid,
    pub repo_uid: Uuid,
}
