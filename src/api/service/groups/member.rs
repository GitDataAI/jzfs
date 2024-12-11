use crate::api::service::groups::GroupService;
use crate::metadata::model::teams::{teams, teams_user};
use sea_orm::*;
use crate::metadata::model::groups::group;

impl GroupService {
    pub async fn member(&self, uid: uuid::Uuid) -> anyhow::Result<Vec<uuid::Uuid>> {
        let team = teams::Entity::find()
            .filter(teams::Column::GroupId.eq(uid))
            .all(&self.db)
            .await?;
        let team = team.into_iter().map(|x| x.uid).collect::<Vec<_>>();
        let mut members = vec![];
        for t in team {
            let member = teams_user::Entity::find()
                .filter(teams_user::Column::TeamId.eq(t))
                .all(&self.db)
                .await?;
            for m in member {
                members.push(m.user_id);
            }
        }
        Ok(members)
    }
    pub async fn check_member(&self, user_id: uuid::Uuid) -> anyhow::Result<Vec<group::Model>> {
        let teams_users = teams_user::Entity::find()
            .filter(teams_user::Column::UserId.eq(user_id))
            .all(&self.db)
            .await?
            .iter()
            .map(|x| x.team_id)
            .collect::<Vec<_>>();
        let teams = teams::Entity::find()
            .filter(group::Column::Uid.is_in(teams_users))
            .all(&self.db)
            .await?
            .iter().map(|x| x.group_id)
            .collect::<Vec<_>>();
        let groups = group::Entity::find()
            .filter(group::Column::Uid.is_in(teams))
            .all(&self.db)
            .await?;
        Ok(groups)
    }
}