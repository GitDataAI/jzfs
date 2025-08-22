use crate::AppCore;
use database::user_interactions;
use database::user_interactions::Interaction;
use sea_orm::*;
use uuid::Uuid;

const INTERACTION_CLONE: f32 = 0.08;
const INTERACTION_STAR: f32 = 0.8;
const INTERACTION_FORK: f32 = 0.5;
const INTERACTION_COMMIT: f32 = 0.02;
const INTERACTION_PR: f32 = 0.4;
const INTERACTION_VIEW: f32 = 0.01;

impl AppCore {
    pub async fn inner_add_interaction(
        &self,
        user_id: Uuid,
        repo_id: Uuid,
        act: Interaction,
    ) -> Result<(), anyhow::Error> {
        let active = user_interactions::ActiveModel {
            uid: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            repo_id: Set(repo_id),
            act: Set(act.clone()),
            created_at: Set(chrono::Local::now().naive_local()),
            weight: Set(match act {
                Interaction::Clone => INTERACTION_CLONE,
                Interaction::Star => INTERACTION_STAR,
                Interaction::Fork => INTERACTION_FORK,
                Interaction::Commit => INTERACTION_COMMIT,
                Interaction::Pr => INTERACTION_PR,
                Interaction::View => INTERACTION_VIEW,
            }),
        };
        user_interactions::Entity::insert(active)
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
