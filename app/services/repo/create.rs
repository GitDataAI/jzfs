use sea_orm::{ActiveModelTrait, QueryFilter, Set, TransactionTrait, TryIntoModel};
use sea_orm::ColumnTrait;
use std::io;
use std::io::Write;
use git2::{ Signature};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;
use crate::app::http::GIT_ROOT;
use crate::app::services::AppState;
use crate::app::services::repo::sync::RepoSync;
use crate::model::repository::repository;
use crate::model::users::users;

#[derive(Deserialize,Serialize)]
pub struct ReposCreateParma {
    pub name: String,
    pub description: Option<String>,
    pub private: bool,
    pub auto_init: bool,
    pub readme: bool,
    pub default_branch: String,
    pub owner: Uuid,
}
impl AppState {
    async fn check_repo_name(&self,users_uid: Uuid, name: String) -> io::Result<bool> {
        let repos = repository::Entity::find()
            .filter(repository::Column::OwnerId.eq(users_uid))
            .filter(repository::Column::Name.eq(name))
            .all(&self.write)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(repos.is_empty())
    }
    pub async fn repo_create(&self,users_uid: Uuid,parma: ReposCreateParma) -> io::Result<()> {
        if !self.check_repo_name(users_uid, parma.name.clone()).await? {
            return Err(io::Error::new(io::ErrorKind::Other, "repository name already exists"));
        }
        let access = self.user_access_owner(users_uid).await?;
        if access.iter().any(|x| x.owner_uid != users_uid) {
            return Err(io::Error::new(io::ErrorKind::Other, "access forbid"));
        }
        let owner_uid = {
            if parma.owner == users_uid {
                users_uid
            } else {
                let owner = access.iter().find(|x| x.owner_uid == parma.owner).unwrap();
                owner.owner_uid
            }
        };
        let txn = self.write.begin().await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let action = repository::ActiveModel {
            uid: Set(Uuid::new_v4()),
            node_uid: Set(Uuid::nil()),
            avatar: Set(None),
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
            owner_id: Set(owner_uid),
            name: Set(parma.name),
            description: Set(parma.description),
            visibility: Set(parma.private),
            fork: Set(None),
            default_branch: Set(parma.default_branch.clone()),
            created_at: Set(chrono::Local::now().naive_local()),
            updated_at: Set(chrono::Local::now().naive_local()),
            http: Set("".to_string()),
            created_by: Set(users_uid),
            topic: Set(Vec::new())
        };
        action.clone().insert(&txn)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let repo = action.try_into_model().unwrap();
        let path = format!("{}/{}/{}", GIT_ROOT, repo.node_uid, repo.uid);
        let par = format!("{}/{}", GIT_ROOT, repo.node_uid);
        if !std::path::Path::new(&par).exists() {
            std::fs::create_dir_all(&par)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        }
        let model = repo.clone();
        let repo = git2::Repository::init_bare(&path)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        repo.set_head(&format!("refs/heads/{}", parma.default_branch)).map_err(
            |e| io::Error::new(io::ErrorKind::Other, e)
        )?;
        if parma.readme {
            let users = users::Entity::find()
                .filter(users::Column::Uid.eq(users_uid))
                .one(&self.read)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                .ok_or(io::Error::new(io::ErrorKind::Other, "user not found"))?;
            let tmp = tempfile::tempdir().map_err(|e|{
                io::Error::new(io::ErrorKind::Other, e)
            })?;
            let repo = git2::Repository::clone(&path, tmp.path())
                .map_err(|e| {
                    io::Error::new(io::ErrorKind::Other, e)
                })?;
            repo.set_head(&format!("refs/heads/{}", parma.default_branch)).map_err(
                |e| io::Error::new(io::ErrorKind::Other, e)
            )?;
            let time = chrono::Local::now().naive_local();
            let time = git2::Time::new(time.and_utc().timestamp(), time.and_utc().timestamp_subsec_nanos() as i32);
            let sig = Signature::new(&users.name, &users.email,&time)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            let mut file = std::fs::File::create(tmp.path().join("README.md"))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            
            file.write_all(b"# README")
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            
            file.flush()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            
            git2::Repository::index(&repo)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                .add_all(["README.md"], git2::IndexAddOption::DEFAULT, None)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            git2::Repository::index(&repo)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                .write()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            let tree_oid = git2::Repository::index(&repo)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                .write_tree()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            let tree = repo.find_tree(tree_oid)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            let commit_oid = git2::Repository::commit(
                &repo,
                None,
                &sig,
                &sig,
                "Initialize Repository",
                &tree,
                &[],
            )
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            git2::Repository::branch(
                &repo,
                &parma.default_branch,
                &repo.find_commit(commit_oid).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
                false,
            )
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            let mut remote = repo.find_remote("origin")
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            let mut callbacks = git2::RemoteCallbacks::new();
            callbacks.push_update_reference(|_,status|{
                if status.is_some() {
                    info!("Push failed");
                    return Err(git2::Error::from_str("Failed to push"));
                }
                info!("Push success");
                Ok(())
            });
            let mut push_options = git2::PushOptions::new();
            push_options.remote_callbacks(callbacks);
            remote.push(&[format!("refs/heads/{}", &parma.default_branch)], Some(&mut push_options))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            tmp.close().ok();
        }
        txn.commit().await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        RepoSync::send(model.uid).await;

        Ok(())
    }
}