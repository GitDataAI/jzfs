use sea_orm::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;
use jz_model::{repository, users};
use crate::AppModule;

#[derive(Deserialize,Serialize)]
pub struct RepositoryListParam {
    pub r#type: String,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub category: Option<String>,
}
#[derive(Deserialize,Serialize)]
pub struct RepositoryListResult {
    pub author: RepositoryListAuthor,
    #[serde(rename = "modelInfo")]
    pub model_info: RepositoryModelInfo,
    #[serde(rename = "moduleStats")]
    pub module_stats: RepositoryStats,
    pub id: Uuid,
}

#[derive(Deserialize,Serialize)]
pub struct RepositoryListAuthor {
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub address: Option<String>
}

#[derive(Deserialize,Serialize)]
pub struct RepositoryModelInfo {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "displayImage")]
    pub display_image: Option<String>,
    pub size: Option<String>,
    pub category: Option<String>,
}

#[derive(Deserialize,Serialize)]
pub struct RepositoryStats {
    pub runs: Option<i32>,
    pub price: Option<i32>,
    #[serde(rename = "APIcompatible")]
    pub api_compatible: bool,
}

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
    pub async fn repo_list(&self, parma: RepositoryListParam) -> anyhow::Result<serde_json::Value> {
        let rtype = if parma.r#type == "all" {
            None
        } else {
            let rf = parma.r#type.to_lowercase();
            if rf == "code" {
                Some("Code".to_string())
            }else if rf == "data" {
                Some("Data".to_string())
            }else if rf == "model" {
                Some("Model".to_string())
            }else { 
                return Err(anyhow::anyhow!("invalid rtype"));
            }
        };
        let mut condition = Condition::all()
            .add(repository::Column::IsPrivate.eq(false));
        if let Some(ref rtype) = rtype {
            condition = condition.add(repository::Column::Rtype.eq(rtype));
        };
        let repos = repository::Entity::find()
            .filter(condition)
            .order_by_desc(repository::Column::CreatedAt)
            .limit(parma.limit.unwrap_or(20) as u64)
            .offset(parma.offset.unwrap_or(0) as u64)
            .all(&self.read)
            .await?;
        let mut result: Vec<RepositoryListResult> = Vec::new();
        for i in repos {
            let author = match users::Entity::find()
                .filter(users::Column::Uid.eq(i.owner_uid))
                .one(&self.read)
                .await? {
                    Some(user) => RepositoryListAuthor {
                        name: Some(user.username),
                        avatar: user.avatar,
                        address: None,
                    },
                    None => match self.org_by_uid(i.owner_uid).await {
                        Ok(org) => RepositoryListAuthor {
                            name: Some(org.name),
                            avatar: org.avatar,
                            address: None,
                        },
                        Err(_) => RepositoryListAuthor {
                            name: None,
                            avatar: None,
                            address: None,
                        },
                    },
                };
            let model_info = RepositoryModelInfo {
                name: i.name,
                version: None,
                description: i.description,
                display_image: None,
                size: None,
                category: None,
            };
            let module_stats = RepositoryStats {
                runs: None,
                price: None,
                api_compatible: false,
            };
            result.push(
                RepositoryListResult {
                    author,
                    model_info,
                    module_stats,
                    id: i.uid,
                },
            );
        }
        let cond = if let Some(ref rtype) = rtype && rtype != "all"{
            Condition::all()
                .add(repository::Column::IsPrivate.eq(false))
                .add(repository::Column::Rtype.eq(rtype))
        } else {
            Condition::all()
                .add(repository::Column::IsPrivate.eq(false))
        };
        let total = repository::Entity::find()
            .filter(cond)
            .count(&self.read)
            .await?;
        Ok(json!({
            "data": result,
            "limit": parma.limit,
            "offset": parma.offset,
            "total": total,
            "type": rtype,
        }))
        
    }
}