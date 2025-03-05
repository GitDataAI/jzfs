use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait};
use std::collections::HashMap;
use std::io;
use serde::{Deserialize, Serialize};
use crate::services::AppState;
use crate::blob::GitBlob;
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
        let repo = self.repo_info(owner, repo).await?;
        tree::Entity::find()
            .filter(tree::Column::RepoUid.eq(repo.uid))
            .filter(tree::Column::Branch.eq(branch))
            .filter(tree::Column::Head.eq(head))
            .one(&self.read)
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Not Found"))
            .map(|x| serde_json::from_str::<crate::blob::tree::GitTree>(&x.content))
            .iter()
            .flatten()
            .collect::<Vec<_>>()
            .first()
            .map(|x|x.to_owned().clone())
            .ok_or(io::Error::new(io::ErrorKind::Other, "Not Found"))
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))
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