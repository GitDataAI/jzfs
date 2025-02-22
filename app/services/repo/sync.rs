use std::collections::HashSet;
use crate::app::services::AppState;
use crate::model::repository::{branches, commits, tree};
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, TransactionTrait};
use sea_orm::{ColumnTrait, IntoActiveModel};
use std::io;
use std::sync::Arc;
use futures::StreamExt;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::OnceCell;
use tracing::{error, info, warn};
use uuid::Uuid;
use crate::model::repository::repository::ActiveModel;

pub static REPO_SYNC: OnceCell<RepoSync> = OnceCell::const_new();

#[derive(Clone)]
pub struct RepoSync {
    pub db: AppState,
    pub rx: UnboundedSender<Uuid>,
}

impl RepoSync {
    pub async fn init() -> Self {
        REPO_SYNC.get_or_init(|| async {
            let (rx, mut tx) = tokio::sync::mpsc::unbounded_channel::<Uuid>();
            let db = AppState::init_env().await.expect("Failed to init app state");
            let db_arc = Arc::new(db.clone());
            tokio::spawn(async move {
                let db_arc = db_arc.clone();
                while let Some(repo_uid) = tx.recv().await {
                    let db_arc = db_arc.clone();
                    match db_arc.repo_sync(repo_uid).await {
                        Ok(_) => info!("Repo sync success {}", repo_uid),
                        Err(e) => error!("Repo sync error: {}", e),
                    }
                }
            });
            Self { db, rx }
        })
        .await.clone()
    }

    pub async fn send(repo_uid: Uuid) {
        info!("Repo sync start {}", repo_uid);
        Self::init().await.rx.send(repo_uid).ok();
    }
}

impl AppState {
    async fn update_repo_model(
        &self,
        mut arch: ActiveModel,
        branch_len: usize,
        default_branch: Option<&str>,
    ) {
        if let Some(name) = default_branch {
            arch.default_branch = Set(name.to_string());
        }
        arch.nums_branch = Set(branch_len as i32);
        arch.updated_at = Set(Utc::now().naive_utc());
        if let Err(e) = arch.update(&self.write).await {
            warn!("Failed to update repo: {}", e);
        }
    }

    pub async fn repo_sync(&self, repo_uid: Uuid) -> io::Result<()> {
        let repo = self.repo_get_by_uid(repo_uid).await?;
        let path = format!(
            "{}/{}/{}",
            crate::app::http::GIT_ROOT,
            repo.node_uid,
            repo.uid
        );
        let blob = crate::blob::GitBlob::new(path.into())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let branches = blob.branch()?;

        let default_exists = branches.iter().any(|x| x.name == repo.default_branch);
        if !default_exists {
            let new_default = branches
                .iter()
                .find(|x| x.name == "main" || x.name == "master")
                .or_else(|| branches.first())
                .map(|x| x.name.as_str());

            self.update_repo_model(
                repo.clone().into_active_model(),
                branches.len(),
                new_default,
            )
            .await;
        }

        let mut latest_timestamp = 0;
        let mut max_commits = 0;
        let branch_data = blob.blob()?;

        for (branch, commits) in branch_data.clone() {
            if let Ok(time) = branch.time.parse::<i64>() {
                latest_timestamp = latest_timestamp.max(time);
            }
            max_commits = max_commits.max(commits.len());

            let branch_uid = match branches::Entity::find()
                .filter(branches::Column::RepoUid.eq(repo_uid))
                .filter(branches::Column::Name.eq(&branch.name))
                .one(&self.read)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            {
                Some(existing) => {
                    let mut model = existing.clone().into_active_model();
                    model.head = Set(branch.head.clone());
                    model.time = Set(branch.time.clone());
                    if let Err(e) = model.update(&self.write).await {
                        warn!("Failed to update branch: {}", e);
                    }
                    existing.uid
                }
                None => branches::ActiveModel {
                    uid: Set(Uuid::new_v4()),
                    repo_uid: Set(repo_uid),
                    protect: Set(false),
                    name: Set(branch.name.clone()),
                    head: Set(branch.head.clone()),
                    time: Set(branch.time),
                }
                .insert(&self.write)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
                .uid,
            };

            let commit_ids: HashSet<_> = commits.iter().map(|c| c.id.clone()).collect();
            let existing_ids: HashSet<_> = {
                let mut ids_stream = commits::Entity::find()
                    .filter(commits::Column::RepoUid.eq(repo_uid))
                    .filter(commits::Column::BranchUid.eq(branch_uid))
                    .filter(commits::Column::Id.is_in(commits.iter().map(|c| &c.id)))
                    .stream(&self.read)
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

                let mut existing = HashSet::new();
                while let Some(Ok(record)) = ids_stream.next().await {
                    existing.insert(record.id);
                }
                existing
            };

            let to_insert: Vec<_> = commits
                .iter()
                .filter(|c| !existing_ids.contains(&c.id))
                .map(|commit| commits::ActiveModel {
                    uid: Set(Uuid::new_v4()),
                    repo_uid: Set(repo_uid),
                    branch_uid: Set(branch_uid),
                    id: Set(commit.id.clone()),
                    message: Set(commit.msg.clone()),
                    time: Set(commit.time.clone()),
                    author: Set(commit.author.clone()),
                    email: Set(commit.email.clone()),
                    status: Set(String::new()),
                    branch_name: Set(branch.name.clone()),
                    runner: Set(vec![]),
                })
                .collect();

            if !to_insert.is_empty() {
                let txn = self
                    .write
                    .begin()
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
                for chunk in to_insert.chunks(1000) {
                    commits::Entity::insert_many(chunk.to_vec())
                        .exec(&txn)
                        .await
                        .map_err(|e| {
                            error!("Batch insert failed: {}", e);
                            io::Error::new(io::ErrorKind::Other, e.to_string())
                        })?;
                }
            
                txn.commit()
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            }
            drop(to_insert);
            drop(existing_ids);
            drop(commit_ids);

            if tree::Entity::find()
                .filter(tree::Column::RepoUid.eq(repo_uid))
                .filter(tree::Column::Branch.eq(&branch.name))
                .filter(tree::Column::Head.eq(&branch.head))
                .one(&self.read)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
                .is_none()
            {
                
                match blob.tree(branch.name.clone()) {
                    Ok(tree) => {
                        let model = tree::ActiveModel {
                            uid: Set(Uuid::new_v4()),
                            repo_uid: Set(repo_uid),
                            head: Set(branch.head.clone()),
                            content: Set(serde_json::to_string(&tree)?),
                            branch: Set(branch.name.clone()),
                        };
                        if let Err(e) = model.insert(&self.write).await {
                            warn!("Failed to insert tree: {}", e);
                        } else {  
                            drop(tree)
                        }
                    }
                    Err(e) => warn!("Failed to get tree: {}", e),
                }
            }
        }
        if max_commits > 0 {
            self.update_repo_model(repo.clone().into_active_model(), branch_data.len(), None)
                .await;
        }
        if latest_timestamp != 0 {
            let mut arch = repo.into_active_model();
            arch.updated_at = Set(
                DateTime::from_timestamp(latest_timestamp, 0)
                    .unwrap()
                    .naive_utc(),
            );
            if let Err(e) = arch.update(&self.write).await {
                warn!("Failed to update timestamp: {}", e);
            }
        }
        drop(blob);
        drop(branch_data);

        Ok(())
    }
}
