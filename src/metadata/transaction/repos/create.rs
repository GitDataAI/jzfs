use log::{error, info};
use sea_orm::ActiveValue::Set;
use sea_orm::*;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::api::dto::repo::RepoCreate;
use crate::metadata::model::groups::{group, group_repo};
use crate::metadata::model::repos::{repo, repo_branch, repo_license};
use crate::metadata::model::users::{users, users_other};
use crate::metadata::transaction::repos::RepoTransaction;
use crate::store::host::GitLocal;

impl RepoTransaction {
    pub async fn create_repo(&self, dto: RepoCreate, created_by: Uuid) -> anyhow::Result<()>{
        let txn = self.db.begin().await.unwrap();
        {
            let mut rootless: Option<String> = None;
            let uid = Uuid::new_v4();
            if dto.is_group{
                let result = group_repo::ActiveModel{
                    uid: Set(Uuid::new_v4()),
                    repo_id: Set(uid),
                    group_id: Set(dto.owner),
                }
                    .insert(&txn)
                    .await;
                match result{
                    Ok(model) => {
                        let model = group::Entity::find_by_id(model.group_id)
                            .one(&txn)
                            .await?;
                        if model.is_none(){
                            txn.rollback().await?;
                            return Err(anyhow::anyhow!("group not found"))
                        }
                        let model = model.unwrap();
                        rootless = Some(model.name);
                    },
                    Err(e) => {
                        txn.rollback().await?;
                        return Err(anyhow::anyhow!("create repo error:{}",e))
                    }
                }
            }else {
                let user_model = users_other::Entity::find()
                    .filter(users_other::Column::UserId.eq(dto.owner))
                    .one(&txn).await?;
                if user_model.is_none(){
                    txn.rollback().await?;
                    return Err(anyhow::anyhow!("user not found"))
                }
                let user_model = user_model.unwrap();
                let mut repos = user_model.repo.clone();
                let mut user_arch = user_model.into_active_model();
                repos.push(uid);
                user_arch.repo = Set(repos);
                let result = user_arch.update(&txn).await;
                match result{
                    Ok(model) => {
                        let model = users::Entity::find_by_id(model.user_id)
                            .one(&txn)
                            .await?;
                        if model.is_none(){
                            txn.rollback().await?;
                            return Err(anyhow::anyhow!("user not found"))
                        }
                        let model = model.unwrap();
                        rootless = Some(model.name);
                    },
                    Err(e) => {
                        txn.rollback().await?;
                        return Err(anyhow::anyhow!("create repo error:{}",e))
                    }
                }
            }
            if rootless.is_none(){
                txn.rollback().await?;
                return Err(anyhow::anyhow!("create repo error:{}", "rootless is none"))
            }
            let rootless = rootless.unwrap();
            let result = repo::ActiveModel{
                uid: Set(uid),
                name: Set(dto.name.clone()),
                description: Set(dto.description),
                commit: Set(0),
                head_hash: Default::default(),
                ssh_path: Set(format!("{}/{}", rootless, dto.name)),
                http_path: Set(format!("{}/{}", rootless, dto.name)),
                star: Set(0),
                fork: Set(0),
                is_fork: Set(false),
                fork_from: Set(None),
                watch: Set(0),
                issue: Set(0),
                open_issue: Set(0),
                close_issue: Set(0),
                pr: Set(0),
                open_pr: Set(0),
                close_pr: Set(0),
                is_empty: Set(true),
                visible: Set(dto.visible),
                topic: Set(dto.topic.unwrap_or(Vec::new())),
                size: Set(0.),
                created_at: Set(OffsetDateTime::now_utc()),
                updated_at: Set(OffsetDateTime::now_utc()),
                created_by: Set(created_by),
            }
                .insert(&txn)
                .await;
            match result{
                Ok(_) => {},
                Err(e) => {
                    txn.rollback().await?;
                    return Err(anyhow::anyhow!("create repo error:{}",e))
                }
            }
            let result = repo_license::ActiveModel{
                uid: Set(Uuid::new_v4()),
                repo_id: Set(uid),
                name: Set(dto.license_name.clone().expect("None")),
                license: Set(dto.license.unwrap_or("None".to_string())),
                created_at: Set(OffsetDateTime::now_utc()),
                updated_at: Set(OffsetDateTime::now_utc()),
                created_by: Set(created_by),
            }
                .insert(&txn)
                .await;
            match result{
                Ok(_) => {},
                Err(e) => {
                    txn.rollback().await?;
                    return Err(anyhow::anyhow!("create repo error:{}",e))
                }
            }
            let result = repo_branch::ActiveModel{
                uid: Set(Uuid::new_v4()),
                repo_id: Set(uid),
                branch: Set(dto.default_branch),
                protect: Set(false),
                visible: Set(true),
                head: Set(None),
                created_at: Set(OffsetDateTime::now_utc()),
                updated_at: Set(OffsetDateTime::now_utc()),
                created_by: Set(created_by),
            }
                .insert(&txn)
                .await;
            match result{
                Ok(_) => {},
                Err(e) => {
                    txn.rollback().await?;
                    return Err(anyhow::anyhow!("create repo error:{}",e))
                }
            }
            GitLocal::init(uid.clone().to_string());
        }
        match txn.commit().await{
            Ok(_) => {
                info!("repo create {} commit success",dto.name)
            },
            Err(e) => {
                error!("repo create {} commit error:{}",dto.name, e)
            }
        }
        Ok(())
    }
}