use crate::app::services::AppState;
use crate::model::repository::{branches, commits, tree};
use chrono::DateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter};
use sea_orm::{ColumnTrait, IntoActiveModel};
use std::io;
use uuid::Uuid;

impl AppState {
    pub async fn repo_sync(&self, repo_uid: Uuid) -> io::Result<()> {
        let repo = self.repo_get_by_uid(repo_uid).await?;
        let path = format!("{}/{}/{}", crate::app::http::GIT_ROOT, repo.node_uid, repo.uid);
        let blob = crate::blob::GitBlob::new(path.into())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;     
        let branch = blob.branch()?;
        
        let mut arch = repo.clone().into_active_model();
        if branch.iter().any(|x|x.name != repo.default_branch) {
            if branch.len() == 1 {
                arch.default_branch = Set(branch[0].name.clone());
                arch.nums_branch = Set(branch.len() as i32);
                arch.updated_at = Set(chrono::Local::now().naive_local());
                arch.update(&self.write).await.ok();
            } else { 
                // find main or master or first branch
                if branch.iter().any(|x|x.name == "main" || x.name == "master") {
                    arch.default_branch = Set(branch[0].name.clone());
                    arch.nums_branch = Set(branch.len() as i32);
                    arch.updated_at = Set(chrono::Local::now().naive_local());
                    arch.update(&self.write).await.ok();
                }else if branch.len() == 1 {
                    if let Some(first) = branch.first() {
                        arch.default_branch = Set(first.name.clone());
                        arch.nums_branch = Set(branch.len() as i32);
                        arch.updated_at = Set(chrono::Local::now().naive_local());
                        arch.update(&self.write).await.ok();
                    }
                }
                
            }
        }
        let mut rec = 0;
        let mut commit_len = 0;
        let branch = blob.blob()?;
        for (branch, commits) in branch.clone() {
            if let Ok(time) = branch.time.parse::<i64>(){
                if time > rec {
                    rec = time;
                }
            }
            if commits.len() > commit_len { 
                commit_len = commits.len();
            }
            let branch_uid = if let Some(x) = branches::Entity::find()
                .filter(branches::Column::RepoUid.eq(repo_uid))
                .filter(branches::Column::Name.eq(branch.name.clone()))
                .one(&self.read)
                .await
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get branches"))?
            {
                let mut xs = x.clone().into_active_model();
                xs.head = Set(branch.head.clone());
                xs.time = Set(branch.time);
                xs.update(&self.write).await.ok();
                x.uid
            } else {
                let res = branches::ActiveModel {
                    uid: Set(Uuid::new_v4()),
                    repo_uid: Set(repo_uid),
                    protect: Set(false),
                    name: Set(branch.name.clone()),
                    head: Set(branch.head.clone()),
                    time: Set(branch.time),
                }
                    .insert(&self.write).await.map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to insert branches"))?;
                res.uid
            };
            for commit in commits {
                if commits::Entity::find()
                    .filter(commits::Column::RepoUid.eq(repo_uid))
                    .filter(commits::Column::BranchUid.eq(branch_uid))
                    .filter(commits::Column::Id.eq(commit.id.clone()))
                    .one(&self.read)
                    .await
                    .map_err(|x| io::Error::new(io::ErrorKind::Other, x.to_string()))?
                    .is_none()
                {
                    let _ = commits::ActiveModel {
                        uid: Set(Uuid::new_v4()),
                        repo_uid: Set(repo_uid),
                        branch_uid: Set(branch_uid),
                        id: Set(commit.id.clone()),
                        message: Set(commit.msg.clone()),
                        time: Set(commit.time),
                        author: Set(commit.author.clone()),
                        email: Set(commit.email.clone()),
                        status: Set(String::new()),
                        branch_name: Set(branch.name.clone()),
                        runner: Set(vec![]),
                    }
                        .insert(&self.write).await;
                }
            }
            
            if tree::Entity::find()
                .filter(tree::Column::RepoUid.eq(repo_uid))
                .filter(tree::Column::Branch.eq(branch.name.clone()))
                .filter(tree::Column::Head.eq(branch.head.clone()))
                .one(&self.read)
                .await
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get tree"))?
                .is_some()
            {
                continue
            }
            let tree = match blob.tree(branch.name.clone()) {
                Ok(tree) => tree,
                Err(_) => {
                    continue
                },
            };
            let _ = tree::ActiveModel {
                uid: Set(Uuid::new_v4()),
                repo_uid: Set(repo_uid),
                head: Set(branch.head),
                content: Set(serde_json::to_string(&tree)?),
                branch: Set(branch.name),
            }
                .insert(&self.write).await;
            
        }
        if commit_len > 0 {
            let mut arch = repo.clone().into_active_model();
            arch.nums_commit = Set(commit_len as i32);
            arch.nums_branch = Set(branch.len() as i32);
            arch.update(&self.write).await.ok();
        }
        if rec != 0 {
            let mut arch = repo.clone().into_active_model();
            arch.nums_branch = Set(branch.len() as i32);
            arch.updated_at = Set(DateTime::from_timestamp(rec, 0).unwrap().naive_local());
            arch.update(&self.write).await.ok();
        }
        Ok(())
    }
}