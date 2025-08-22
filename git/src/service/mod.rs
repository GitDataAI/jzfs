use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use config::AppConfig;
use database::user_interactions;
use database::user_interactions::Interaction;
use sea_orm::prelude::Uuid;
use sea_orm::sqlx::types::chrono;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

#[derive(Clone)]
pub struct GitServer {
    pub db: DatabaseConnection,
    pub config: AppConfig,
    pub redis: Pool<RedisConnectionManager>,
}

pub mod find;
pub mod permissions;
pub mod sync;

impl GitServer {
    pub async fn inner_add_interaction_clone(
        &self,
        user_id: Uuid,
        repo_id: Uuid,
    ) -> Result<(), anyhow::Error> {
        let active = user_interactions::ActiveModel {
            uid: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            repo_id: Set(repo_id),
            act: Set(Interaction::Clone),
            created_at: Set(chrono::Local::now().naive_local()),
            weight: Set(0.08),
        };
        user_interactions::Entity::insert(active)
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
