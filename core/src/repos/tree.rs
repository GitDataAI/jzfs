use crate::AppCore;
use error::AppError;
use git::GitContext;
use git::object::tree::{TreeItemLastCommit, TreeParam};
use redis::AsyncCommands;

impl AppCore {
    pub async fn repos_tree(
        &self,
        namespace: &str,
        repo_name: &str,
        refs: Option<String>,
        tree_oid: Option<String>,
        dir: String,
    ) -> Result<Vec<TreeItemLastCommit>, AppError> {
        let cache_key = format!(
            "repo:tree:cache:{}:{}:{}:{}:{}",
            namespace,
            repo_name,
            repo_name,
            dir.clone(),
            tree_oid.clone().unwrap_or("".to_string())
        );
        if let Ok(mut conn) = self.redis.get().await {
            if let Ok(result) = conn.get::<String, String>(cache_key.clone()).await {
                if let Ok(result) = serde_json::from_str::<Vec<TreeItemLastCommit>>(&result) {
                    return Ok(result);
                }
            }
        }
        let repo = self.repo_find(namespace, repo_name).await?;
        let git = GitContext::try_from((repo, self.config.git.clone()))?;
        let tree = git.tree(TreeParam {
            refs,
            tree_oid,
            dir,
        })?;
        let tree = git.tree_item_last_commit(tree)?;
        if let Ok(mut conn) = self.redis.get().await {
            if let Ok(result) = serde_json::to_string(&tree) {
                conn.set_ex::<String, String, ()>(cache_key, result, 60)
                    .await
                    .ok();
            }
        }
        Ok(tree)
    }
}
