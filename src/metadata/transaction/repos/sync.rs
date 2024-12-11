use sea_orm::*;
use sea_orm::ActiveValue::Set;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::metadata::model::repos::{repo, repo_branch, repo_commit};
use crate::metadata::transaction::repos::RepoTransaction;
use crate::store::host::GitLocal;

impl RepoTransaction {
    pub async fn async_repo_branch_commit(&self, repo_id: Uuid) -> anyhow::Result<()> {
        let repo = repo::Entity::find_by_id(repo_id)
            .one(&self.db)
            .await?;
        
        if repo.is_none(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        
        let repo = repo.unwrap();
        let txn = self.db.begin().await.unwrap();
        {
            let commit = repo_commit::Entity::find()
                .filter(
                    repo_commit::Column::RepoId.eq(repo_id)
                )
                .all(&txn)
                .await?;
            let branch = repo_branch::Entity::find()
                .filter(
                    repo_branch::Column::RepoId.eq(repo_id)
                )
                .all(&txn)
                .await?;
            {
                for c in commit{
                    let arch = c.into_active_model();
                    let result = arch.delete(&txn).await;
                    match result{
                        Ok(_) => {},
                        Err(e) => {
                            txn.rollback().await?;
                            return Err(anyhow::anyhow!("delete repo error:{}",e))
                        }
                    }
                }
                for b in branch.clone(){
                    let arch = b.into_active_model();
                    let result = arch.delete(&txn).await;
                    match result{
                        Ok(_) => {},
                        Err(e) => {
                            txn.rollback().await?;
                            return Err(anyhow::anyhow!("delete repo error:{}",e))
                        }
                    }
                }
            }
            {
                let store = GitLocal::init(repo.uid.to_string());
                let branchs = store.branchs()?;
                let mut brs = vec![];
                for b in branchs{
                    if b.is_empty() {
                        continue
                    }
                    let last = b
                        .split("/")
                        .last()
                        .map(|x| x.to_string());
                    if last.is_none(){
                        continue
                    }
                    let last = last.unwrap();
                    let md = branch.iter().find(|x| x.branch == last).map(|x| x.clone());
                    let result = repo_branch::ActiveModel {
                        uid: Set(Uuid::new_v4()),
                        repo_id: Set(repo_id),
                        branch: Set(last.clone()),
                        protect: Set(md.map(|x| x.protect).unwrap_or(false)),
                        visible: Set(true),
                        head: Set(None),
                        created_at: Set(OffsetDateTime::now_utc()),
                        updated_at: Set(OffsetDateTime::now_utc()),
                        created_by: Set(repo.created_by),
                    }
                        .insert(&txn)
                        .await;
                    match result{
                        Ok(model) => {
                            brs.push(model);
                        },
                        Err(e) => {
                            txn.rollback().await?;
                            return Err(anyhow::anyhow!("delete repo error:{}",e))
                        }
                    }
                        
                }
                for b in brs{
                    let commit = store.commits_history(b.branch);
                    if commit.is_err(){
                        txn.rollback().await?;
                        return Err(anyhow::anyhow!("delete repo error:{}",commit.err().unwrap()))
                    }
                    let commit = commit.unwrap();
                    for c in commit{
                        let result = repo_commit::ActiveModel {
                            uid: Set(Uuid::new_v4()),
                            repo_id: Set(repo_id),
                            branch_id: Set(b.uid),
                            bio: Set(c.message),
                            commit_user: Set(c.author),
                            commit_email: Set(c.email),
                            commit_id: Set(c.hash),
                            created_at: Set(c.date),
                        }
                            .insert(&txn)
                            .await;
                        match result{
                            Ok(_) => {},
                            Err(e) => {
                                txn.rollback().await?;
                                return Err(anyhow::anyhow!("delete repo error:{}",e))
                            }
                        }
                    }
                }
            }
        }
        txn.commit().await?;
        Ok(())
    }
}