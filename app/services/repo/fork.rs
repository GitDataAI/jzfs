use sea_orm::{ActiveModelTrait, QueryFilter, TransactionTrait, TryIntoModel};
use sea_orm::ColumnTrait;
use std::io;
use chrono::Utc;
use sea_orm::{EntityTrait, Set};
use uuid::Uuid;
use crate::app::http::GIT_ROOT;
use crate::app::services::AppState;
use crate::app::services::statistics::repo::FORK;
use crate::model::repository::repository;

#[derive(serde::Deserialize)]
pub struct ForkParma {
    pub owner_uid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub private: bool,
}


impl AppState {
    pub async fn repo_fork(&self, users_uid: Uuid, repo_uid: Uuid, parma: ForkParma) -> io::Result<()> {
        let access = self.user_access_owner(users_uid).await?;
        let txn = self.write.begin().await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
        if access.iter().any(|x| x.owner_uid != parma.owner_uid) {
            return Err(io::Error::new(io::ErrorKind::Other, "you can't fork your own repository"));
        }
        if let Some(repo) = repository::Entity::find()
            .filter(repository::Column::Uid.eq(repo_uid))
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
        {
            let fork = repository::ActiveModel {
                uid: Set(Uuid::new_v4()),
                owner_id: Set(users_uid),
                name: Set(parma.name),
                description: Set(parma.description),
                visibility: Set(!parma.private),
                created_at: Set(Utc::now().naive_utc()),
                fork: Set(Some(repo.uid)),
                nums_fork: Set(0),
                nums_star: Set(0),
                nums_watch: Set(0),
                nums_issue: Set(0),
                nums_pullrequest: Set(0),
                nums_commit: Set(0),
                nums_release: Set(0),
                nums_tag: Set(0),
                nums_branch: Set(0),
                ssh: Set("".to_string()),
                default_branch: Set(repo.default_branch.clone()),
                node_uid: Set(Uuid::nil()),
                updated_at: Set(chrono::Local::now().naive_local()),
                http: Set("".to_string()),
                created_by: Set(users_uid),
                topic: Set(repo.topic),
                avatar: Set(None),
            };
            fork.clone().insert(&txn).await
                .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
            let path = format!("{}/{}/{}/", GIT_ROOT, repo.node_uid, repo.uid);
            let fork_model = fork.try_into_model().unwrap();
            let new_path = format!("{}/{}/{}/", GIT_ROOT, fork_model.node_uid, fork_model.uid);
            // Copy dir
            copy_dir::copy_dir(path, new_path)?;
            self.statistics_repo(repo.uid, FORK.to_string()).await.ok();
            self.repo_sync(fork_model.uid).await.ok();
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "repo not found"));
        }
        txn.commit().await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
        Ok(())
    }
}