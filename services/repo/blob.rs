use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait};
use std::collections::HashMap;
use std::io;
use deadpool_redis::redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use crate::services::AppState;
use crate::blob::GitBlob;
use crate::blob::tree::GitTree;
use crate::model::repository::{branches, commits, tree};

impl AppState {
    pub async fn repo_blob_bhct(&self, owner: String, repo: String) ->  io::Result<HashMap<branches::Model,Vec<commits::Model>>> {
        let repo = self.repo_info(owner.clone(), repo).await?;
        let branch = self.branch_list(owner, repo.name).await?;
        let mut map = HashMap::new();
        for branch in branch {
            let commit = commits::Entity::find()
                .filter(commits::Column::RepoUid.eq(repo.uid))
                .filter(commits::Column::BranchName.eq(branch.name.clone()))
                .filter(commits::Column::BranchUid.eq(branch.uid))
                .all(&self.read)
                .await
                .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
            map.insert(branch, commit);
        }
        Ok(map)
    }
    pub async fn repo_blob_tree(&self, owner: String, repo: String, branch: String, head: String) -> io::Result<crate::blob::tree::GitTree> {
        let key = format!("{}/{}/{}/{}", owner,repo,branch,head);
        if let Ok(mut x) = self.cache.lock() {
            if let Ok(xs) = x.get::<String, String>(key.clone()).await {
                x.expire::<String, String>(key, 60).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                return serde_json::from_str(&xs)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
            }
            drop(x);
        }
        let repo = self.repo_info(owner, repo.clone()).await?;
        let entry = tree::Entity::find()
            .filter(tree::Column::RepoUid.eq(repo.uid))
            .filter(tree::Column::Branch.eq(branch.clone()))
            .filter(tree::Column::Head.eq(head.clone()))
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Tree entry not found"))?;
        let result:GitTree = serde_json::from_str(&entry.content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        if let Ok(mut x) = self.cache.lock() {
            x.set::<String, String, String>(key.clone(), entry.content).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            x.expire::<String, String>(key, 60).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            drop(x);
        }
        
        Ok(result)
        
    }

    pub async fn repo_blob_file(&self, param: RepoBlobFile) -> io::Result<Vec<u8>> {
        let RepoBlobFile { owner, repo, paths, sha } = param;
        let repo = self.repo_info(owner, repo).await?;
        let path = format!("{}/{}/{}", crate::http::GIT_ROOT, repo.node_uid, repo.uid);
        let blob = GitBlob::new(path.into())
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
        blob.file(paths,sha)
    }
}

#[derive(Deserialize,Serialize,Clone)]
pub struct RepoBlobFile {
    pub owner: String,
    pub repo: String,
    pub paths: String,
    pub sha: String,
}