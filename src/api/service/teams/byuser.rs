use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use crate::api::service::teams::TeamService;
use crate::metadata::model::teams::{teams, teams_user};

impl TeamService {
    pub async fn byuser(&self, uid: Uuid) -> anyhow::Result<Vec<teams::Model>> {
        let models = teams_user::Entity::find()
            .filter(
                teams_user::Column::UserId.eq(uid)
            )
            .all(&self.db)
            .await?
            .iter()
            .map(|x| x.team_id)
            .collect::<Vec<_>>();
        let models = teams::Entity::find()
            .filter(
                teams::Column::Uid.is_in(models)
            )
            .all(&self.db)
            .await?;
        Ok(models)
    }
}