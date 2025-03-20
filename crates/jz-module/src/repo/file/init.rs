use crate::AppModule;
use anyhow::Context;
use jz_model::repository;
use jz_model::repository::RepositoryInitParam;
use sea_orm::{ActiveModelTrait, TransactionTrait};
use std::path::PathBuf;
use uuid::Uuid;

impl AppModule {
    pub async fn repo_init(&self, ops_uid: Uuid, param: RepositoryInitParam) -> anyhow::Result<()> {
        let users = self
            .user_info_by_id(ops_uid)
            .await
            .context("You are who? Who are you? Please contact Us")?;
        if !users.allow_create {
            return Err(anyhow::anyhow!("You are not allow to create repository"))?;
        }
        let owner_repo = self.repo_info_by_owner_uid(param.owner_uid).await?;
        if owner_repo.len() as i32 >= users.max_repository {
            return Err(anyhow::anyhow!(
                "The maximum number of warehouses you can create has been reached"
            ))?;
        }
        if owner_repo.iter().any(|x| x.name == param.repo_name) {
            return Err(anyhow::anyhow!("Repository name already exists"))?;
        }
        let txn = self.write.begin().await?;
        let active = repository::ActiveModel::new(param.clone());
        active.clone().insert(&txn).await?;
        let uid = active.uid.clone().unwrap();
        let node = active.node.unwrap();
        let mut git = jz_git::GitParam::new(
            PathBuf::from("./data").join(node.to_string()),
            uid.to_string(),
        )?;
        git.repo_create(true)?;
        if param.readme {
            if param.default_branch.is_none() {
                return Err(anyhow::anyhow!("Please specify the default branch"))?;
            }
            let file = jz_git::commits::CreateGitCommit {
                branches: param.default_branch.unwrap(),
                user: users.username,
                email: users.email,
                msg: "Initial commit".to_string(),
                path: "".to_string(),
                file: "README.md".to_string(),
                ops: 1,
                context: format!(r"# {}", param.repo_name).as_bytes().to_vec(),
            };
            git.create_commit(file)?;
        }
        txn.commit().await?;
        Ok(())
    }
}
