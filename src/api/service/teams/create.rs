use sea_orm::*;
use uuid::Uuid;
use crate::api::dto::team::TeamCreate;
use crate::api::service::teams::TeamService;
use crate::metadata::model::teams::{teams, teams_user};

impl TeamService {
    pub async fn create_team(&self, dto: TeamCreate, owner: Uuid, group: Uuid) -> anyhow::Result<()>{
        let txn = self.db.begin().await.unwrap();
        let uid = Uuid::new_v4();
        
        teams::ActiveModel {
            uid: Set(uid),
            name: Set(dto.name),
            description: Set(dto.description),
            group_id: Set(group),
            created_at: Set(time::OffsetDateTime::now_utc()),
            updated_at: Set(time::OffsetDateTime::now_utc()),
            created_by: Set(owner),
        }
            .save(&txn)
            .await?;
        teams_user::ActiveModel {
            uid: Set(Uuid::new_v4()),
            team_id: Set(uid),
            user_id: Set(owner),
            join_at: Set(time::OffsetDateTime::now_utc()),
            access: Set(3),
        }
            .save(&txn)
            .await?;
        txn.commit().await?;
        Ok(())
    }
}