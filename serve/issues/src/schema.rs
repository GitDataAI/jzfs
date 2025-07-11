use anyhow::Error;
use bytes::buf::Limit;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueFetchParam {
    pub issue_id: i32,
    pub repo_uid: Uuid,
    pub state: Option<String>,
    pub key: Option<String>,
    pub page: u64,
    pub limit: u64
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueUpdateParam {
    
    pub new_title: Option<String>,
    pub new_description: Option<String>
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueCreateParam {
    pub title: String,
    pub description: Option<String>,
    pub labels: Vec<Uuid>,
    pub assignee_uid: Vec<Uuid>,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueCommentFetchParam {
    pub issue_id: i32,
    pub repo_uid: Uuid,
    pub comment_id: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueCommentUpdateParam {
    pub issue_id: i32,
    pub comment_uid: Uuid,
    pub update_content: Option<String>,
    pub author_uid: Uuid,
    pub repo_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueCommentCreateParam {
    pub content: String,
    pub parent_comment_uid: Option<Uuid>
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueLabelFetchParam {
    pub repo_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueLabelUpdateParam {
    pub repo_uid: Uuid,
    pub update_label_uid: Uuid,
    pub label_color: Option<String>,
    pub label_name: Option<String>
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueLabelCreateParam {
    pub issue_uid: Uuid,
    pub repo_uid: Uuid,
    pub label_name: String,
    pub label_color: String,
    pub label_uid : Uuid,
    pub user_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueLabelLinkParam {
    pub issue_id: i32,
    pub label_uid: Uuid,
    pub limit: u64,
    pub page: u64
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueLabelUnlinkParam {
    pub issue_id: i32,
    pub label_uid: Uuid,
    pub repo_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueLabelDeleteParam {
    pub label_id: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueSubscribeParam {
    pub issue_id: i32,
    pub subscribe_uid: Uuid,
    pub is_subscribed: bool,
    pub user_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueStatusUpdateParam {
    pub status: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueAssigneeUpdateParam {
    pub update_assignee_uid: Vec<Uuid>,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct IssueHistoryFetchParam {
    pub issue_uid: Uuid,
    pub repo_uid: Uuid,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct LabelCreateParam {
    pub repo_uid: Uuid,
    pub label_name: String,
    pub label_color: String,
    pub description: Option<String>,
}


