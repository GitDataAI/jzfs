use sea_orm::ColumnTrait;
use std::io;
use sea_orm::{EntityTrait, QueryFilter};
use crate::services::AppState;
use crate::model::repository::commits;

impl AppState {
    pub async fn repo_commit_one(&self, owner: String, repo: String, _: String, sha: String) -> io::Result<commits::Model> {
        let repo = self.repo_info(owner, repo).await?;
        let mut commits = commits::Entity::find()
            .filter(commits::Column::Id.eq(sha))
            .filter(commits::Column::RepoUid.eq(repo.uid))
            .all(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get commit 0"))?;
        commits.sort_by(|a, b| b.time.cmp(&a.time));
        match commits.first() {
            Some(commit) => Ok(commit.clone()),
            None => Err(io::Error::new(io::ErrorKind::Other, "Failed to get commit 1"))
        }
    }
}