use crate::AppCore;
use anyhow::anyhow;
use database::entity::users;
use database::user_interactions::Interaction;
use error::AppError;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};
use serde_json::json;
use session::Session;

impl AppCore {
    pub async fn repos_data(
        &self,
        namespace: &str,
        repo_name: &str,
        session: Session,
    ) -> Result<serde_json::Value, AppError> {
        let repo = self.repo_find(namespace, repo_name).await?;
        let mut value = serde_json::Value::Null;
        value["model"] = json!(repo);
        let user = self.user_context(session).await;

        if let Ok(user) = user.clone() {
            self.inner_add_interaction(user.user_uid, repo.uid, Interaction::View)
                .await?;
        }
        let namespace = repo.namespace;
        let owner = users::Entity::find()
            .filter(users::Column::Username.eq(namespace))
            .one(&self.db)
            .await?;
        if let Some(owner) = owner {
            value["owner"] = json!({
                "uid": owner.uid,
                "username": owner.username,
                "display_name": owner.display_name,
                "avatar": owner.avatar_url,
                "team": false,
            });
            if repo.is_private {
                if let Ok(user) = user {
                    if owner.uid == user.user_uid {
                        value["is_owner"] = json!(true);
                    } else {
                        return Err(AppError::from(anyhow!("No access permission")));
                    }
                    if let Some(_) = database::user_star_repo::Entity::find()
                        .filter(database::user_star_repo::Column::UserId.eq(user.user_uid))
                        .filter(database::user_star_repo::Column::RepoId.eq(repo.uid))
                        .one(&self.db)
                        .await?
                    {
                        value["is_star"] = json!(true);
                    } else {
                        value["is_star"] = json!(false);
                    }
                    if let Some(_) = database::user_watch_repo::Entity::find()
                        .filter(database::user_watch_repo::Column::UserId.eq(user.user_uid))
                        .filter(database::user_watch_repo::Column::RepoId.eq(repo.uid))
                        .one(&self.db)
                        .await?
                    {
                        value["is_watch"] = json!(true);
                    } else {
                        value["is_watch"] = json!(false);
                    }
                } else {
                    value["is_owner"] = json!(false);
                }
            } else {
                if let Ok(user) = user {
                    if owner.uid == user.user_uid {
                        value["is_owner"] = json!(true);
                    } else {
                        value["is_owner"] = json!(false);
                    }
                } else {
                    value["is_owner"] = json!(false);
                }
            }
        }
        let state = database::git_repo_stats::Entity::find()
            .filter(database::git_repo_stats::Column::RepoUid.eq(repo.uid))
            .one(&self.db)
            .await?;
        if let Some(state) = state {
            value["state"] = json!({
                "watches": state.watches,
                "stars": state.stars,
                "forks": state.forks,
            });
        } else {
            value["state"] = json!({
                "watches": 0,
                "stars": 0,
                "forks": 0,
            });
        }
        Ok(value)
    }
}
