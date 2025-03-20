use crate::AppModule;
use jz_model::{repository, star, users};
use sea_orm::*;
use uuid::Uuid;

impl AppModule {
    pub async fn repo_star(
        &self,
        ops_uid: Uuid,
        owner: String,
        repo: String,
    ) -> anyhow::Result<()> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let user = self.user_info_by_id(ops_uid).await?;
        // todo access

        let mut repo_active = repo.clone().into_active_model();
        if star::Entity::find()
            .filter(
                Condition::all()
                    .add(star::Column::UserId.eq(user.uid))
                    .add(star::Column::RepositoryId.eq(repo.uid)),
            )
            .one(&self.write)
            .await?
            .is_some()
        {
            star::Entity::delete_many()
                .filter(
                    Condition::all()
                        .add(star::Column::UserId.eq(user.uid))
                        .add(star::Column::RepositoryId.eq(repo.uid)),
                )
                .exec(&self.write)
                .await?;
            repo_active.nums_star = Set(repo.nums_star - 1);
            repo_active.update(&self.write).await?;
        } else {
            star::ActiveModel::new(user.uid, repo.uid)
                .insert(&self.write)
                .await?;
            repo_active.nums_star = Set(repo.nums_star + 1);
            repo_active.update(&self.write).await?;
        }
        Ok(())
    }
    pub async fn repo_star_list_by_user(
        &self,
        ops_uid: Uuid,
    ) -> anyhow::Result<Vec<repository::Model>> {
        let user = self.user_info_by_id(ops_uid).await?;
        let list = star::Entity::find()
            .filter(star::Column::UserId.eq(user.uid))
            .all(&self.read)
            .await?;
        let mut result = Vec::new();
        for item in list {
            let is = repository::Entity::find_by_id(item.repository_id)
                .one(&self.read)
                .await?;
            if is.is_some() {
                result.push(is.unwrap());
            }
        }
        Ok(result)
    }
    pub async fn repo_star_list_by_repo(
        &self,
        ops_uid: Uuid,
        owner: String,
        repo: String,
    ) -> anyhow::Result<Vec<serde_json::Value>> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let user = self.user_info_by_id(ops_uid).await?;
        if repo.owner_uid != user.uid {
            return Err(anyhow::anyhow!("no permission"));
        }
        // todo GROUP

        let value = serde_json::Value::Null;
        let list = star::Entity::find()
            .filter(star::Column::RepositoryId.eq(repo.uid))
            .all(&self.read)
            .await?;
        let mut result = Vec::new();
        for item in list {
            let is = users::Entity::find_by_id(item.user_id)
                .one(&self.read)
                .await?;
            let unix = item.created_at.and_utc().timestamp();
            if let Some(user) = is {
                result.push(serde_json::json!({
                    "username": user.username,
                    "avatar": user.avatar,
                    "uid": user.uid,
                    "description": user.description,
                    "created_unix": unix,
                }));
            } else {
                result.push(value.clone());
            }
        }
        Ok(result)
    }
}
