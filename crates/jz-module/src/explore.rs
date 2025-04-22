use serde_json::json;
use jz_model::{member, organization, repository, users};
use crate::AppModule;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct ExploreParma {
    pub page: i64,
    pub size: i64,
    pub filter: String,
    pub search: String,
    pub rtype: String, // repo | user | org
}


impl AppModule {
    pub async fn explore(&self, param: ExploreParma) -> anyhow::Result<serde_json::Value> {
        let mut value = serde_json::Value::Null;
        if param.rtype == "product" {
            value["type"] = json!("product");
            let mut data = vec![];
            let mut condition = Condition::all()
                .add(repository::Column::IsPrivate.eq(false));
            if param.search != "" {
                condition = condition.add(repository::Column::Name.contains(param.search.clone()))
                    .add(repository::Column::Description.contains(param.search))
            }
            let query = repository::Entity::find()
                .filter(condition.clone())
                .order_by_desc(repository::Column::CreatedAt)
                .offset((param.page - 1) as u64 * param.size as u64)
                .limit(param.size as u64)
                .all(&self.read)
                .await?;
            for idx in query {
                let owner = match self.user_info_by_id(idx.owner_uid).await {
                    Ok(owner) => json!({
                        "uid": owner.uid,
                        "name": owner.username,
                        "avatar": owner.avatar,
                        "description": owner.description,
                    }),
                    Err(_)=> {
                        match self.org_by_uid(idx.owner_uid).await {
                            Ok(org) => json!({
                                "uid": org.uid,
                                "name": org.name,
                                "avatar": org.avatar,
                                "description": org.description,
                            }),
                            Err(_) => continue,
                        }
                    }
                };
                let v = json!({
                    "uid": idx.uid,
                    "owner": owner,
                    "name": idx.name,
                    "description": idx.description,
                    "created_at": idx.created_at,
                    "updated_at": idx.updated_at,
                    "star": idx.nums_star,
                    "fork": idx.nums_fork,
                    "watch": idx.nums_watch,
                    "topic": idx.topic,
                    "rtype": idx.rtype,
                    "default_branch": idx.default_branch,
                });
                data.push(v);
            }
            value["data"] = json!(data);
            let total = repository::Entity::find()
                .filter(condition)
                .count(&self.read)
                .await?;
            value["total"] = json!(total);
            return Ok(value);
        }else if param.rtype == "user" {
            value["type"] = json!("user");
            let mut data = vec![];
            let mut cond = Condition::all();
            if !param.search.is_empty() {
                cond = cond.add(users::Column::Username.contains(param.search.clone()))
                    .add(users::Column::Description.contains(param.search))
            }
            let query = users::Entity::find()
                .filter(cond.clone())
                .order_by_desc(users::Column::CreatedAt)
                .offset((param.page - 1) as u64 * param.size as u64)
                .limit(param.size as u64)
                .all(&self.read)
                .await?;
            for idx in query {
                let repo = repository::Entity::find()
                    .filter(repository::Column::OwnerUid.eq(idx.uid))
                    .count(&self.read)
                    .await?;
                let v = json!({
                    "uid": idx.uid,
                    "name": idx.username,
                    "avatar": idx.avatar,
                    "repo": repo,
                    "followed": idx.nums_fans,
                    "following": idx.nums_following,
                    "description": idx.description,
                    "created_at": idx.created_at,
                    "updated_at": idx.updated_at,
                });
                data.push(v);
            }
            value["data"] = json!(data);
            let total = users::Entity::find()
                .filter(cond)
                .count(&self.read)
                .await?;
            value["total"] = json!(total);
            return Ok(value);
        }else if param.rtype == "organization" {
            value["type"] = json!("organization");
            let mut data = vec![];
            let mut cond = Condition::all();
            if !param.search.is_empty() {
                cond = cond.add(organization::Column::Name.contains(param.search.clone()))
                    .add(organization::Column::Description.contains(param.search))
            }
            let query = organization::Entity::find()
                .filter(cond.clone())
                .order_by_desc(organization::Column::CreatedAt)
                .offset((param.page - 1) as u64 * param.size as u64)
                .limit(param.size as u64)
                .all(&self.read)
                .await?;
            
            for idx in query {
                let member = member::Entity::find()
                    .filter(member::Column::GroupUid.eq(idx.uid))
                    .count(&self.read)
                    .await?;
                let repo = repository::Entity::find()
                    .filter(repository::Column::OwnerUid.eq(idx.uid))
                    .count(&self.read)
                    .await?;
                let v = json!({
                    "uid": idx.uid,
                    "name": idx.name,
                    "avatar": idx.avatar,
                    "member": member,
                    "repo": repo,
                    "description": idx.description,
                    "created_at": idx.created_at,
                    "updated_at": idx.updated_at,
                });
                data.push(v);
            }
            value["data"] = json!(data);
            let total = organization::Entity::find()
                .filter(cond)
                .count(&self.read)
                .await?;
            value["total"] = json!(total);
            return Ok(value);
        }
        return Err(anyhow::anyhow!("rtype error"))
    }
}