use crate::AppModule;
use jz_git::tree::{GitTreeParam, TreeEntityItem};
use jz_git::tree_dir::GitCommitTree;
use uuid::Uuid;
use jz_dragonfly::Pool;
use jz_dragonfly::redis::AsyncCommands;

impl AppModule {
    pub async fn repo_tree(
        &self,
        ops_uid: Option<Uuid>,
        owner: String,
        repo: String,
        param: GitTreeParam,
    ) -> anyhow::Result<Vec<TreeEntityItem>> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        // if repo.is_private {
        //     if let Some(ops_uid) = ops_uid {
        //         let user = self.user_info_by_id(ops_uid).await?;
        //         if user.uid != repo.owner_uid {
        //             return Err(anyhow::anyhow!("permission denied"));
        //         }
        //     } else {
        //         return Err(anyhow::anyhow!("permission denied"));
        //     }
        // }
        let mut git = repo.git()?;
        let tree = git.tree(param)?;

        Ok(tree)
    }
    pub async fn repo_tree_message(
        &self,
        ops_uid: Option<Uuid>,
        owner: String,
        repo_name: String,
        param: GitTreeParam,
    ) -> anyhow::Result<Vec<GitCommitTree>> {
        let repo = self.repo_info_by_owner_and_name(owner.clone(), repo_name.clone()).await?;
        // if repo.is_private {
        //     if let Some(ops_uid) = ops_uid {
        //         let user = self.user_info_by_id(ops_uid).await?;
        //         if user.uid != repo.owner_uid {
        //             return Err(anyhow::anyhow!("permission denied"));
        //         }
        //     } else {
        //         return Err(anyhow::anyhow!("permission denied"));
        //     }
        //     // TODO GROUP
        // }
        let mut git = repo.git()?;
        let cache_key = format!("owner:{},repo:{},sha: {:?},branch: {:?}", owner, repo_name, param.sha, param.branches);
        let cache = self.ioc.take::<Pool>().await;
        if let Some(cache) = cache.clone() {
            let mut cache = cache.get().await?;
            let json = cache.get::<String, String>(cache_key.clone()).await;
            let _ = cache.expire::<String, i64>(cache_key.clone(), 60 * 60 * 24).await;
            if let Ok(json) = json {
                return Ok(serde_json::from_str(&json)?);
            }
        };
        let tree = git.tree_msg(param.branches.clone())?;
        if let Some(cache) = cache {
            let mut cache = cache.get().await?;
            let _ = cache.set::<String, String,String>(cache_key.clone(), serde_json::to_string(&tree)?).await;
            let _ = cache.expire::<String, i64>(cache_key, 60 * 60 * 24).await;
        };
        Ok(tree)
    }
}
