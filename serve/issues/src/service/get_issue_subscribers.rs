use cert::schema::AppResult;
use issuesd::{issue_sub, issues};
use crate::schema::IssueLabelLinkParam;
use crate::service::AppIssueService;
use sea_orm::*;
use serde_json::json;
use uuid::Uuid;

impl AppIssueService {
    pub async fn service_get_issue_subscribers(&self, param: IssueLabelLinkParam,repo_uid: Uuid,subscribe_uid: Uuid) -> AppResult<serde_json::Value> {
        let result = issue_sub::Entity::find()
            .filter(
                Condition::all()
                    .add(issue_sub::Column::IssueUid.eq(param.issue_id))
                    .add(issue_sub::Column::SubscriberUid.eq(subscribe_uid))
            )
            .order_by_desc(issue_sub::Column::CreatedAt)
            .offset(param.limit * param.page)
            .limit(param.limit)
            .all(&self.db)
            .await
            .unwrap_or(vec![]);
        let issues_uid = result.iter().map(|issue| issue.issue_uid).collect::<Vec<_>>();
        let issues = issues::Entity::find()
            .filter(
                Condition::all()
                    .add(issues::Column::Uid.is_in(issues_uid))
                    .add(issues::Column::RepoUid.eq(repo_uid))
            )
            .all(&self.db)
            .await
            .unwrap_or(vec![]);
        let size = issue_sub::Entity::find()
            .filter(issue_sub::Column::SubscriberUid.eq(subscribe_uid))
            .count(&self.db)
            .await
            .unwrap_or(result.len() as u64);
        
        AppResult {
            code: 200,
            data: Some(json!({
                "size": size,
                "issues": issues,
                "page": param.page,
                "limit": param.limit,
            })),
            msg: None,
        }
    }
    
}