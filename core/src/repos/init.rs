use crate::AppCore;
use config::git::AppGitStorage;
use database::entity::{git_repo, user_repo};
use database::git_repo_stats;
use error::AppError;
use git::GitContext;
use sea_orm::prelude::Uuid;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use session::Session;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RepoInitParam {
    pub owner_uid: Uuid,
    pub repo_name: String,
    pub repo_description: String,
    pub repo_is_private: bool,
    pub repo_default_branch: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RepoOwnerSelectItem {
    pub uid: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub team: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RepoInitBefore {
    pub owner_uid: Uuid,
    pub team: bool,
    pub repo_name: String,
}

impl AppCore {
    pub async fn repo_init_select_owner(
        &self,
        session: Session,
    ) -> Result<Vec<RepoOwnerSelectItem>, AppError> {
        let user = self.user_context(session).await?;
        let mut result = Vec::new();
        result.push(RepoOwnerSelectItem {
            uid: user.user_uid,
            username: user.username,
            display_name: user.display_name,
            avatar: user.avatar_url,
            team: false,
        });
        // TODO team
        Ok(result)
    }
    pub async fn repo_init_select_storage(&self) -> Result<Vec<AppGitStorage>, AppError> {
        let storage = self.config.git.storage.clone();
        Ok(storage)
    }
    pub async fn repo_init_before(&self, param: RepoInitBefore) -> Result<(), AppError> {
        if !param.team {
            let owner_repo = user_repo::Entity::find()
                .filter(user_repo::Column::UserUid.eq(param.owner_uid))
                .all(&self.db)
                .await?
                .iter()
                .map(|x| x.repo_uid)
                .collect::<Vec<_>>();
            let repo = git_repo::Entity::find()
                .filter(git_repo::Column::Uid.is_in(owner_repo))
                .all(&self.db)
                .await?;
            if repo.iter().any(|x| x.repo_name == param.repo_name) {
                return Err(AppError::from(anyhow::anyhow!(
                    "The repository already exists"
                )));
            };
        } else {
            // TODO
        }
        Ok(())
    }
    pub async fn repo_init_main(
        &self,
        param: RepoInitParam,
        session: Session,
    ) -> Result<(), AppError> {
        let select = self.repo_init_select_owner(session).await?;
        let txn = self.db.begin().await?;
        if let Some(owner) = select.iter().find(|x| x.uid == param.owner_uid) {
            let repo_uid = Uuid::now_v7();
            let repo = git_repo::ActiveModel {
                uid: Set(repo_uid),
                namespace: Set(owner.username.clone()),
                repo_name: Set(param.repo_name),
                default_head: Set(param.repo_default_branch.clone()),
                is_private: Set(param.repo_is_private),
                created_at: Set(Utc::now().naive_utc()),
                updated_at: Set(Utc::now().naive_utc()),
                storage: Set("default".to_string()),
                description: Set(if param.repo_description.is_empty() {
                    None
                } else {
                    Some(param.repo_description)
                }),
            };
            let stats = git_repo_stats::ActiveModel {
                uid: Set(Uuid::now_v7()),
                repo_uid: Set(repo_uid),
                stars: Set(0),
                watches: Set(0),
                forks: Set(0),
                created_at: Set(Utc::now().naive_utc()),
                updated_at: Set(Utc::now().naive_utc()),
            };
            stats.insert(&txn).await?;
            let model = repo.insert(&txn).await?;
            if !owner.team {
                let user_repo = user_repo::ActiveModel {
                    uid: Set(Uuid::now_v7()),
                    repo_uid: Set(model.uid),
                    user_uid: Set(owner.uid),
                };
                user_repo.insert(&txn).await?;
            } else {
                // TODO
            }
            let git = GitContext::try_from((model, self.config.git.clone()))?;
            git.init()?;
            git.refs_exchange_head(&format!("refs/heads/{}", param.repo_default_branch))?;
        };
        txn.commit().await?;
        Ok(())
    }
}
