use crate::AppModule;
use jz_model::{repository, star, watch};
use sea_orm::*;
use serde_json::json;
use uuid::Uuid;

impl AppModule {
    pub async fn repo_info_by_owner_and_name(
        &self,
        owner: String,
        name: String,
    ) -> anyhow::Result<repository::Model> {
        repository::Entity::find()
            .filter(
                Condition::all()
                    .add(repository::Column::OwnerName.eq(owner))
                    .add(repository::Column::Name.eq(name)),
            )
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("not found"))
    }
    pub async fn repo_info_by_id(&self, uid: Uuid) -> anyhow::Result<repository::Model> {
        repository::Entity::find_by_id(uid)
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("not found"))
    }
    pub async fn repo_info_by_owner(
        &self,
        owner: String,
    ) -> anyhow::Result<Vec<repository::Model>> {
        Ok(repository::Entity::find()
            .filter(repository::Column::OwnerName.eq(owner))
            .all(&self.read)
            .await?)
    }
    pub async fn repo_info_by_owner_uid(
        &self,
        owner_uid: Uuid,
    ) -> anyhow::Result<Vec<repository::Model>> {
        Ok(repository::Entity::find()
            .filter(repository::Column::OwnerUid.eq(owner_uid))
            .all(&self.read)
            .await?)
    }
    
    pub async fn repo_info_data(&self, ops_uid: Option<Uuid>, owner_name: String, repo_name: String) -> anyhow::Result<serde_json::Value> {
        let repo = self.repo_info_by_owner_and_name(owner_name.clone(), repo_name.clone()).await?;
        let branch = self.repo_list_branch(ops_uid, owner_name,repo_name).await?
            .iter().map(|x|{
            json!({
                "name": x.name,
                "is_head": x.is_head,
                "upstream": x.upstream,
                "active": x.active.clone().map(|x|{
                    json!({
                        "id": x.id,
                        "msg": x.msg,
                        "author": x.author,
                        "email": x.email,
                        "date": x.date,
                    })
                }),
            })
        })
        .collect::<Vec<_>>();
        let star = if let Some(ops_uid) = ops_uid {
           let user = self.user_info_by_id(ops_uid).await?;
            if star::Entity::find()
                .filter(
                    Condition::all()
                        .add(star::Column::UserId.eq(user.uid))
                        .add(star::Column::RepositoryId.eq(repo.uid)),
                )
                .one(&self.read)
                .await?
                .is_some()
            {
                true
            } else {
                false
            }
        } else {
            false
        };
        let watch = if let Some(ops_uid) = ops_uid {
            let user = self.user_info_by_id(ops_uid).await?;
            if watch::Entity::find()
                .filter(
                    Condition::all()
                        .add(watch::Column::UserId.eq(user.uid))
                        .add(watch::Column::RepoUid.eq(repo.uid)),
                )
                .one(&self.read)
                .await?
                .is_some()
            {
                true
            } else {
                false
            }
        } else {
            false
        };
        let owner = match self.user_info_by_id(repo.owner_uid).await {
            Ok(owner) => json!({
                 "uid": owner.uid,
                "username": owner.username,
                "avatar": owner.avatar,
                "created_at": owner.created_at,
                "updated_at": owner.updated_at,
            }),
            Err(_)=> {
                match self.org_by_uid(repo.owner_uid).await {
                    Ok(org) => json!({
                         "uid": org.uid,
                        "name": org.name,
                        "avatar": org.avatar,
                        "description": org.description,
                        "created_at": org.created_at,
                        "updated_at": org.updated_at,
                    }),
                    Err(_) => json!({
                        "uid": 0,
                    })
                }
            }
        };
        let setting = if let Some(ops_uid) = ops_uid {
            if repo.owner_uid == ops_uid {
                true
            }else {
                let access = self.repo_access(ops_uid).await?;
                if access.iter().any(|x| x.access >= 50) {
                    true
                } else {
                    false
                }
            }
        }else { 
            false
        };
        Ok(json!({
            "uid": repo.uid,
            "owner_name": repo.owner_name,
            "owner_uid": repo.owner_uid,
            "repo_name": repo.name,
            "description": repo.description,
            "is_private": repo.is_private,
            "topic": repo.topic,
            "rtype": repo.rtype,
            "default_branch": repo.default_branch,
            "node": repo.node,
            "created_at": repo.created_at,
            "updated_at": repo.updated_at,
            "nums_star": repo.nums_star,
            "nums_fork": repo.nums_fork,
            "nums_watch": repo.nums_watch,
            "nums_branch": branch.len(),
            "owner": owner,
            "branch": branch,
            "website": repo.website,
            "star": star,
            "watch": watch,
            "setting": setting,
        }))
    }
    
    pub async fn repo_can_setting(&self, ops_uid: Option<Uuid>, owner_name: String, repo_name: String) -> anyhow::Result<bool> {
        if let Some(ops_uid) = ops_uid {
            let repo = self.repo_info_by_owner_and_name(owner_name.clone(), repo_name.clone()).await?;
            if repo.owner_uid == ops_uid {
                return Ok(true);
            }
            let access = self.repo_access(ops_uid).await?;
            if access.iter().any(|x| x.access >= 50) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
