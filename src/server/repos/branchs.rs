use crate::error::{JZError, JZResult};
use crate::models::repos::{branchs, repos};
use crate::server::MetaData;
use sea_orm::prelude::Expr;
use sea_orm::*;
use uuid::Uuid;

impl MetaData {
    pub async fn repo_branchs(&self, owner: String, name: String) -> JZResult<Vec<branchs::Model>> {
        let result = repos::Entity::find()
            .filter(repos::Column::Owner.eq(owner))
            .filter(repos::Column::Name.eq(name))
            .one(&self.database)
            .await?;
        if result.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[033] Repo Not Found")));
        }
        let result = branchs::Entity::find()
            .filter(branchs::Column::RepoId.eq(result.unwrap().uid))
            .all(&self.database)
            .await?;
        Ok(result)
    }
    pub async fn repo_branch_info(
        &self,
        owner: String,
        name: String,
        branch: String,
    ) -> JZResult<branchs::Model> {
        let result = repos::Entity::find()
            .filter(repos::Column::Owner.eq(owner))
            .filter(repos::Column::Name.eq(name))
            .one(&self.database)
            .await?;
        if result.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[034] Repo Not Found")));
        }
        if let Some(branch) = branchs::Entity::find()
            .filter(
                Condition::all()
                    .add(branchs::Column::RepoId.eq(result.unwrap().uid))
                    .add(branchs::Column::Name.eq(branch)),
            )
            .one(&self.database)
            .await?
        {
            return Ok(branch);
        }
        Err(JZError::Other(anyhow::anyhow!("[035] Branch Not Found")))
    }
    pub async fn repo_default_branch_get(
        &self,
        owner: String,
        name: String,
    ) -> JZResult<branchs::Model> {
        let result = repos::Entity::find()
            .filter(repos::Column::Owner.eq(owner))
            .filter(repos::Column::Name.eq(name))
            .one(&self.database)
            .await?;
        if result.is_none() {
            return Err(JZError::Other(anyhow::anyhow!("[036] Repo Not Found")));
        }
        if let Some(branch) = branchs::Entity::find()
            .filter(branchs::Column::RepoId.eq(result.unwrap().uid))
            .one(&self.database)
            .await?
        {
            return Ok(branch);
        }
        Err(JZError::Other(anyhow::anyhow!("[037] Branch Not Found")))
    }
    pub async fn repo_default_branch(
        &self,
        owner: String,
        name: String,
        branch: String,
    ) -> JZResult<()> {
        let info = match self.repo_info(owner, name).await {
            Ok(info) => info,
            Err(e) => return Err(e),
        };
        let txn = self.database.begin().await?;
        let mut arch = info.into_active_model();
        arch.default_branchs = ActiveValue::Set(Some(branch));
        arch.updated = ActiveValue::Set(chrono::Local::now().timestamp());
        match arch.update(&txn).await {
            Ok(_) => {
                txn.commit().await?;
                Ok(())
            }
            Err(e) => {
                txn.rollback().await?;
                Err(JZError::Other(anyhow::anyhow!(
                    "[040] Repo Default Branch Failed: {}",
                    e.to_string()
                )))
            }
        }
    }
    pub async fn repo_branch_sync(&self, repo_id: Uuid) -> JZResult<()> {
        match repos::Entity::find_by_id(repo_id)
            .one(&self.database)
            .await?
        {
            Some(model) => model,
            None => return Err(JZError::Other(anyhow::anyhow!("[048] Repo NotFound "))),
        };
        let local_repo = match self.git.open_repo(repo_id.to_string()) {
            Ok(repo) => repo,
            Err(e) => {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[049] Open Repo Failed: {}",
                    e.to_string()
                )))
            }
        };
        let txn = self.database.begin().await?;
        let branchs = local_repo.branch_list()?;
        for branch in branchs {
            if let Some(br) = branchs::Entity::find()
                .filter(
                    Condition::all()
                        .add(branchs::Column::RepoId.eq(repo_id))
                        .add(branchs::Column::Name.eq(branch.name.clone()))
                        .add(branchs::Column::Head.eq(branch.head.clone())),
                )
                .one(&self.database)
                .await?
            {
                if let Some(head) = br.head.clone() {
                    if head == branch.head {
                        continue;
                    } else {
                        br.into_active_model().delete(&txn).await.ok();
                        let _ = branchs::ActiveModel {
                            uid: ActiveValue::Set(Uuid::new_v4()),
                            repo_id: ActiveValue::Set(repo_id),
                            name: ActiveValue::Set(branch.name),
                            head: ActiveValue::Set(Option::from(branch.head)),
                            protect: ActiveValue::Set(false),
                        }
                            .insert(&txn)
                            .await;
                    }
                }
            }else {
                let _ = branchs::ActiveModel {
                    uid: ActiveValue::Set(Uuid::new_v4()),
                    repo_id: ActiveValue::Set(repo_id),
                    name: ActiveValue::Set(branch.name),
                    head: ActiveValue::Set(Option::from(branch.head)),
                    protect: ActiveValue::Set(false),
                }
                    .insert(&txn)
                    .await;
            }
        }
        txn.commit().await?;
        Ok(())
    }
    pub async fn repo_branch_delete(
        &self,
        owner: String,
        name: String,
        branch: String,
    ) -> JZResult<()> {
        let info = match self.repo_info(owner, name).await {
            Ok(info) => info,
            Err(e) => return Err(e),
        };
        if let Some(default) = info.default_branchs.clone() {
            if branch == default {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[041] Repo Default Branch Can Not Delete"
                )));
            }
        }
        let result = match branchs::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(branchs::Column::RepoId.eq(info.uid))
                    .add(branchs::Column::Name.eq(branch.clone())),
            )
            .exec(&self.database)
            .await
        {
            Ok(result) => result,
            Err(e) => {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[042] Repo Branch Delete Failed: {}",
                    e.to_string()
                )))
            }
        };
        let local = match self.git.open_repo(info.uid.to_string()) {
            Ok(repo) => repo,
            Err(e) => {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[044] Open Repo Failed: {}",
                    e.to_string()
                )))
            }
        };

        if result.rows_affected == 0 {
            Err(JZError::Other(anyhow::anyhow!(
                "[043] Repo Branch Not Found"
            )))
        } else {
            let result = local.branch_delete(&branch);
            if result.is_err() {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[045] Repo Branch Delete Failed: {}",
                    result.err().unwrap().to_string()
                )));
            }
            Ok(())
        }
    }
    pub async fn repo_branch_protect(
        &self,
        owner: String,
        name: String,
        branch: String,
    ) -> JZResult<()> {
        let info = match self.repo_info(owner, name).await {
            Ok(info) => info,
            Err(e) => return Err(e),
        };
        if let Some(default) = info.default_branchs.clone() {
            if branch == default {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[046] Repo Default Branch Can Not Protect"
                )));
            }
        }
        match branchs::Entity::update_many()
            .col_expr(branchs::Column::Protect, Expr::value(true))
            .filter(
                Condition::all()
                    .add(branchs::Column::RepoId.eq(info.uid))
                    .add(branchs::Column::Name.eq(branch.clone())),
            )
            .exec(&self.database)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(JZError::Other(anyhow::anyhow!(
                "Repo Branch Protect Failed: {}",
                e.to_string()
            ))),
        }
    }
    pub async fn repo_branch_unprotect(
        &self,
        owner: String,
        name: String,
        branch: String,
    ) -> JZResult<()> {
        let info = match self.repo_info(owner, name).await {
            Ok(info) => info,
            Err(e) => return Err(e),
        };
        if let Some(default) = info.default_branchs.clone() {
            if branch == default {
                return Err(JZError::Other(anyhow::anyhow!(
                    "[047] Repo Default Branch Can Not UnProtect"
                )));
            }
        }
        match branchs::Entity::update_many()
            .col_expr(branchs::Column::Protect, Expr::value(false))
            .filter(
                Condition::all()
                    .add(branchs::Column::RepoId.eq(info.uid))
                    .add(branchs::Column::Name.eq(branch.clone())),
            )
            .exec(&self.database)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(JZError::Other(anyhow::anyhow!(
                "[048] Repo Branch UnProtect Failed: {}",
                e.to_string()
            ))),
        }
    }
}
