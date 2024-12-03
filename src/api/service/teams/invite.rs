use sea_orm::*;
use uuid::Uuid;
use crate::api::service::teams::TeamService;
use crate::metadata::model::teams::teams_invite;

impl TeamService {
    pub async fn invite_user(&self, group: Uuid, team: Uuid, user: Uuid, email: String) -> anyhow::Result<()>{
        let txn = self.db.begin().await?;
        teams_invite::ActiveModel {
            uid: Set(Uuid::new_v4()),
            group_id: Set(group),
            team_id: Set(team),
            user_id: Set(user),
            email: Set(email),
            status: Set(0),
            created_at: Set(time::OffsetDateTime::now_utc()),
            updated_at: Set(time::OffsetDateTime::now_utc()),
            invited_by: Set(user),
        }
            .save(&txn)
            .await?;
        txn.commit().await?;
        Ok(())
    }
}