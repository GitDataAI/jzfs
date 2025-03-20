use crate::AppModule;
use jz_model::{repository, watch};
use sea_orm::*;
use uuid::Uuid;

impl AppModule {
    pub async fn repo_watch(
        &self,
        ops_uid: Uuid,
        owner: String,
        repo: String,
        level: i32,
    ) -> anyhow::Result<()> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let user = self.user_info_by_id(ops_uid).await?;
        let mut repo_active = repo.clone().into_active_model();
        if watch::Entity::find()
            .filter(watch::Column::UserId.eq(user.uid))
            .filter(watch::Column::RepoUid.eq(repo.uid))
            .one(&self.write)
            .await?
            .is_some()
        {
            watch::Entity::delete_many()
                .filter(watch::Column::UserId.eq(user.uid))
                .filter(watch::Column::RepoUid.eq(repo.uid))
                .exec(&self.write)
                .await?;
            repo_active.nums_watch = Set(repo.nums_watch - 1);
            repo_active.update(&self.write).await?;
        } else {
            watch::ActiveModel::new_with_user_id(user.uid, repo.uid, level)
                .insert(&self.write)
                .await?;
            repo_active.nums_watch = Set(repo.nums_watch + 1);
            repo_active.update(&self.write).await?;
        }
        Ok(())
    }
    pub async fn repo_watch_list_by_user(
        &self,
        ops_uid: Uuid,
    ) -> anyhow::Result<Vec<repository::Model>> {
        let result = watch::Entity::find()
            .filter(watch::Column::UserId.eq(ops_uid))
            .all(&self.read)
            .await?;
        let mut res = vec![];
        for i in result {
            let item = repository::Entity::find_by_id(i.repo_uid)
                .one(&self.read)
                .await?;
            if item.is_some() {
                res.push(item.unwrap());
            }
        }
        Ok(res)
    }
    pub async fn repo_watch_list_by_repo(
        &self,
        ops_uid: Uuid,
        owner: String,
        repo: String,
    ) -> anyhow::Result<Vec<serde_json::Value>> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let user = self.user_info_by_id(ops_uid).await?;
        if user.uid != repo.owner_uid {
            return Err(anyhow::anyhow!("permission denied"));
        }
        // TODO GROUP
        let result = watch::Entity::find()
            .filter(watch::Column::RepoUid.eq(repo.uid))
            .all(&self.read)
            .await?;
        let mut res = vec![];
        for i in result {
            let item = self.user_info_by_id(i.user_id).await?;
            res.push(serde_json::json!({
                "level": i.level,
                "avatar": item.avatar,
                "username": item.username,
                "uid": item.uid,
            }));
        }
        Ok(res)
    }
}
