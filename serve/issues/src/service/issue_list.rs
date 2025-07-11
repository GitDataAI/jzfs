use crate::schema::IssueFetchParam;
use crate::service::AppIssueService;
use cert::schema::AppResult;
use issuesd::issues;
use sea_orm::*;
use serde_json::json;
impl AppIssueService {
    pub async fn service_get_issue_list(&self, param: IssueFetchParam) -> AppResult<serde_json::Value> {
        let result = issues::Entity::find()
            .filter(
                Condition::all()
                    .add(issues::Column::RepoUid.eq(param.repo_uid))
                    .add_option(param.state.map(|state| issues::Column::Status.contains(state)))
            )
            .order_by_desc(issues::Column::IssueId)
            .offset(param.limit * param.page)
            .limit(param.limit)
            .all(&self.db)
            .await
            .unwrap_or(vec![]);
        let size = issues::Entity::find()
            .all(&self.db)
            .await.unwrap_or_else(|_| vec![]).len();
        AppResult {
            code: 200,
            data: Some(json!({
                "size": size,
                "page": param.page,
                "limit": param.limit,
                "issues": result
            })),
            msg: None,
        }
    }
 }