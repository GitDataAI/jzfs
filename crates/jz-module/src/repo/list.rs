use serde_json::{json, Value};
use uuid::Uuid;
use crate::AppModule;

impl AppModule {
    pub async fn repo_list_info(&self,ops_uid: Option<Uuid>, owner_name: String) -> anyhow::Result<Vec<Value>> {
        let repos = self.repo_info_by_owner(owner_name).await?;
        
        let mut result = vec![];
        for i in repos {
            result.push(json!({
                "uid": i.uid,
                "owner_name": i.owner_name,
                "owner_uid": i.owner_uid,
                "repo_name": i.name,
                "description": i.description,
                "is_private": i.is_private,
                "topic": i.topic,
                "rtype": i.rtype,
                "default_branch": i.default_branch,
                "node": i.node,
                "created_at": i.created_at,
                "updated_at": i.updated_at,
                "owner": if let Some(ops_uid) = ops_uid {
                    ops_uid == i.owner_uid
                } else {
                    false
                },
                "nums_star": i.nums_star,
                "nums_fork": i.nums_fork,
                "nums_watch": i.nums_watch,
                "nums_branch": i.nums_branch,
                "nums_issue": i.nums_issue,
            }));
        }
        Ok(result)
    }
}