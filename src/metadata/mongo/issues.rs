use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;


#[derive(Deserialize,Serialize,Clone,Debug,ToSchema)]
pub struct IssuesModel{
    pub issues_id: Uuid,
    pub repo_id: String,
    pub title: String,
    pub body: String,
    pub created_by: Uuid,
    pub created_username: String,
    pub assignees: Vec<IssuesAssignees>,
    pub labels: Vec<IssuesLabels>,
    pub comments: Vec<IssuesComment>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub closed_at: Option<i64>,
    pub closed: bool,
    pub notifications: Vec<Uuid>,
    pub participant: Vec<IssuesAssignees>,
}

#[derive(Deserialize,Serialize,Clone,Debug,ToSchema)]
pub struct IssuesComment{
    pub comment_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub avatar: Option<String>,
    pub body: String,
    pub role: String,
    pub created_at: i64,
    pub reply: Option<Box<Vec<IssuesComment>>>,
}

#[derive(Deserialize,Serialize,Clone,Debug,ToSchema)]
pub struct IssuesLabels{
    pub color: String,
    pub name: String,
    pub add_at: u64,
}

#[derive(Deserialize,Serialize,Clone,Debug,ToSchema)]
pub struct IssuesAssignees{
    pub user_id: Uuid,
    pub name: String,
    pub avatar: String,
    pub add_at: u64,
}

#[derive(Deserialize,Serialize,Clone,Debug,ToSchema)]
pub struct IssuesEndpoint {
    pub title: String,
    pub participant: IssuesAssignees,
    pub add_at: u64,
}