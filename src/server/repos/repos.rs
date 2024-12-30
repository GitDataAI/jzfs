use crate::error::{JZError, JZResult};
use crate::models::groups::groups;
use crate::models::repos::repos;
use crate::models::repos::repos::RepoCreateOptions;
use crate::models::teams::{teamrepo, teams};
use crate::models::users::users;
use crate::server::MetaData;
use anyhow::anyhow;
use sea_orm::prelude::Expr;
use sea_orm::*;
use uuid::Uuid;

impl MetaData {
    pub async fn repo_create(&self, options: RepoCreateOptions) -> JZResult<()> {
        let txn = self.database.begin().await?;
        let repo_id = Uuid::new_v4();
        let owner = options.owner.clone();
        let name = options.name.clone();
        if options.is_group {
            let result = teams::Entity::find()
                .filter(teams::Column::Uid.eq(options.owner_id))
                .one(&self.database)
                .await?;
            if result.is_none() {
                txn.rollback().await?;
                return Err(JZError::Other(anyhow::anyhow!("[028] Group Not Found")));
            }
            let groups_id = result.unwrap().org_id;
            let result = repos::Entity::find()
                .filter(repos::Column::Owner.eq(owner.clone()))
                .filter(repos::Column::Name.eq(name.clone()))
                .one(&self.database)
                .await?;
            if result.is_some() {
                txn.rollback().await?;
                return Err(JZError::Other(anyhow::anyhow!("[029] Repo Already Exists")));
            }
            let result = teamrepo::ActiveModel {
                uid: Set(Uuid::new_v4()),
                repo_id: Set(repo_id),
                team_id: Set(result.unwrap().uid),
                access: Set(7),
                created: Set(chrono::Local::now().timestamp()),
            }
            .insert(&txn)
            .await;
            if result.is_err() {
                txn.rollback().await?;
                return Err(JZError::Other(anyhow::anyhow!("[030] Repo Create Failed")));
            }
            groups::Entity::update_many()
                .filter(groups::Column::Uid.eq(groups_id))
                .col_expr(
                    groups::Column::Repository,
                    Expr::add(Expr::col(groups::Column::Repository), 1),
                )
                .exec(&txn)
                .await?;
        } else {
            let result = users::Entity::update_many()
                .filter(users::Column::Uid.eq(options.owner_id))
                .col_expr(
                    users::Column::Repository,
                    Expr::add(Expr::col(users::Column::Repository), 1),
                )
                .exec(&txn)
                .await;
            if result.is_err() {
                txn.rollback().await?;
                return Err(JZError::Other(anyhow::anyhow!("[031] Repo Create Failed")));
            }
        }
        let result = repos::ActiveModel {
            uid: Set(repo_id),
            owner: Set(options.owner),
            owner_id: Set(options.owner_id),
            avatar_url: Set(None),
            name: Set(options.name),
            description: Set(options.description),
            website: Set(None),
            private: Set(options.private),
            is_group: Set(options.is_group),
            has_issues: Set(true),
            has_idcard: Set(true),
            has_wiki: Set(true),
            has_downloads: Set(true),
            has_projects: Set(true),
            topic: Set(Vec::new()),
            collaborators: Set(Vec::new()),
            git_http_url: Set(format!("0x_http_x0/{}/{}.git", owner, name)),
            git_ssh_url: Set(format!("0x_ssh_x0/{}/{}.git", owner, name)),
            default_branchs: Set(None),
            nums_star: Set(0),
            nums_fork: Set(0),
            nums_watcher: Set(0),
            nums_commit: Set(0),
            nums_release: Set(0),
            nums_tag: Set(0),
            nums_branchs: Set(0),
            nums_members: Set(0),
            fork: Set(false),
            fork_from: Set(None),
            created: Set(chrono::Local::now().timestamp()),
            updated: Set(chrono::Local::now().timestamp()),
        }
        .insert(&txn)
        .await;
        if result.is_err() {
            txn.rollback().await?;
            return Err(JZError::Other(anyhow::anyhow!("[027] Repo Create Failed")));
        }
        match self.git.create_repo(repo_id.to_string()) {
            Ok(_) => {
                txn.commit().await?;
                Ok(())
            }
            Err(e) => {
                txn.rollback().await?;
                Err(JZError::Other(anyhow!(e.to_string())))
            }
        }
    }
    pub async fn repo_info(&self, owner: String, name: String) -> JZResult<repos::Model> {
        let result = repos::Entity::find()
            .filter(repos::Column::Owner.eq(owner))
            .filter(repos::Column::Name.eq(name))
            .one(&self.database)
            .await?;
        if result.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[032] Repo Not Found")));
        }
        Ok(result.unwrap())
    }
    pub async fn repo_rename(&self, owner: String, name: String, new_name: String) -> JZResult<()> {
        let info = match self.repo_info(owner, name).await {
            Ok(info) => info,
            Err(e) => return Err(e),
        };
        let txn = self.database.begin().await?;
        let mut arch = info.into_active_model();
        arch.name = sea_orm::ActiveValue::Set(new_name);
        arch.updated = sea_orm::ActiveValue::Set(chrono::Local::now().timestamp());
        match arch.update(&txn).await {
            Ok(_) => {}
            Err(e) => {
                txn.rollback().await?;
                return Err(JZError::Other(anyhow::anyhow!(
                    "[037] Repo Rename Failed: {}",
                    e.to_string()
                )));
            }
        }
        Ok(())
    }
    pub async fn repo_delete(&self, owner: String, name: String) -> JZResult<()> {
        let info = match self.repo_info(owner, name).await {
            Ok(info) => info,
            Err(e) => return Err(e),
        };
        let txn = self.database.begin().await?;
        match repos::Entity::delete_by_id(info.uid).exec(&txn).await {
            Ok(_) => {}
            Err(e) => {
                txn.rollback().await?;
                return Err(JZError::Other(anyhow::anyhow!(
                    "[038] Repo Delete Failed: {}",
                    e.to_string()
                )));
            }
        }
        match self.git.remove_repo(info.uid.to_string()) {
            Ok(_) => {}
            Err(e) => {
                txn.rollback().await?;
                return Err(JZError::Other(anyhow::anyhow!(
                    "[039] Repo Delete Failed: {}",
                    e.to_string()
                )));
            }
        }

        txn.commit().await?;
        Ok(())
    }
    pub async fn repo_avatar(
        &self,
        owner: String,
        name: String,
        avatar_url: String,
    ) -> JZResult<()> {
        let info = match self.repo_info(owner, name).await {
            Ok(info) => info,
            Err(e) => return Err(e),
        };
        let txn = self.database.begin().await?;
        let mut arch = info.into_active_model();
        arch.avatar_url = sea_orm::ActiveValue::Set(Some(avatar_url));
        arch.updated = sea_orm::ActiveValue::Set(chrono::Local::now().timestamp());
        match arch.update(&txn).await {
            Ok(_) => {
                txn.commit().await?;
                Ok(())
            }
            Err(e) => {
                txn.rollback().await?;
                Err(JZError::Other(anyhow::anyhow!(
                    "[039] Repo Avatar Failed: {}",
                    e.to_string()
                )))
            }
        }
    }
    pub async fn repo_description(
        &self,
        owner: String,
        name: String,
        description: String,
    ) -> JZResult<()> {
        let info = match self.repo_info(owner, name).await {
            Ok(info) => info,
            Err(e) => return Err(e),
        };
        let txn = self.database.begin().await?;
        let mut arch = info.into_active_model();
        arch.description = sea_orm::ActiveValue::Set(Some(description));
        arch.updated = sea_orm::ActiveValue::Set(chrono::Local::now().timestamp());
        match arch.update(&txn).await {
            Ok(_) => {
                txn.commit().await?;
                Ok(())
            }
            Err(e) => {
                txn.rollback().await?;
                Err(JZError::Other(anyhow::anyhow!(
                    "[041] Repo Description Failed: {}",
                    e.to_string()
                )))
            }
        }
    }
    pub async fn repo_collaborators(
        &self,
        owner: String,
        name: String,
        user_id: Uuid,
        add: bool,
    ) -> JZResult<()> {
        let info = match self.repo_info(owner, name).await {
            Ok(info) => info,
            Err(e) => return Err(e),
        };
        let mut collaborators = info.collaborators.clone();
        if add {
            if collaborators.contains(&user_id) {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[042] Repo Collaborators Already Exists"
                )));
            }
            collaborators.push(user_id);
        } else {
            if !collaborators.contains(&user_id) {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[043] Repo Collaborators Not Found"
                )));
            }
            collaborators.retain(|x| *x != user_id);
        }
        let txn = self.database.begin().await?;
        let mut arch = info.into_active_model();
        arch.collaborators = sea_orm::ActiveValue::Set(collaborators);
        arch.updated = sea_orm::ActiveValue::Set(chrono::Local::now().timestamp());
        match arch.update(&txn).await {
            Ok(_) => {
                txn.commit().await?;
                Ok(())
            }
            Err(e) => {
                txn.rollback().await?;
                Err(JZError::Other(anyhow::anyhow!(
                    "[044] Repo Collaborators Failed: {}",
                    e.to_string()
                )))
            }
        }
    }
    pub async fn repo_search(&self, keyword: String) -> JZResult<Vec<repos::Model>> {
        let result = repos::Entity::find()
            .filter(
                Condition::any()
                    .add(repos::Column::Name.contains(&keyword))
                    .add(repos::Column::Description.contains(&keyword)),
            )
            .all(&self.database)
            .await?;
        Ok(result)
    }
}
