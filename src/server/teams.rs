use crate::error::{JZError, JZResult};
use crate::models::teams::teams::TeamCreateOption;
use crate::models::teams::{teams, teamsus};
use crate::server::MetaData;
use anyhow::anyhow;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::*;
use uuid::Uuid;

impl MetaData {
    pub async fn teams_create(
        &self,
        option: TeamCreateOption,
        created_by: Uuid,
    ) -> JZResult<teams::Model> {
        let team = teams::ActiveModel {
            uid: Set(Uuid::new_v4()),
            org_id: Set(option.org_id),
            name: Set(option.name),
            description: Set(option.description),
            created: Set(chrono::Local::now().timestamp()),
            updated: Set(chrono::Local::now().timestamp()),
            created_by: Set(created_by),
        }
        .insert(&self.database)
        .await;
        match team {
            Ok(team) => Ok(team),
            Err(err) => Err(err.into()),
        }
    }
    pub async fn teams_delete(&self, uid: Uuid) -> JZResult<()> {
        let team = teams::Entity::delete_by_id(uid).exec(&self.database).await;
        match team {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
    pub async fn teams_update(
        &self,
        uid: Uuid,
        name: Option<String>,
        description: Option<String>,
    ) -> JZResult<()> {
        let team = teams::Entity::update_many()
            .col_expr(
                teams::Column::Name,
                SimpleExpr::from(name.unwrap_or("".to_string())),
            )
            .col_expr(
                teams::Column::Description,
                SimpleExpr::from(description.unwrap_or("".to_string())),
            )
            .filter(teams::Column::Uid.eq(uid))
            .exec(&self.database)
            .await;
        match team {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
    pub async fn teams_get(&self, uid: Uuid) -> JZResult<teams::Model> {
        let team = teams::Entity::find_by_id(uid).one(&self.database).await?;
        match team {
            Some(team) => Ok(team),
            None => Err(JZError::Other(anyhow!("[66] Team Not Found"))),
        }
    }
    pub async fn teams_list(&self, org_id: Uuid) -> JZResult<Vec<teams::Model>> {
        let team = teams::Entity::find()
            .filter(teams::Column::OrgId.eq(org_id))
            .all(&self.database)
            .await;
        match team {
            Ok(team) => Ok(team),
            Err(e) => Err(JZError::Other(anyhow!("[67] {}", e.to_string()))),
        }
    }
    pub async fn teams_list_by_user(&self, user_id: Uuid) -> JZResult<Vec<teams::Model>> {
        let teams = teamsus::Entity::find()
            .filter(teamsus::Column::UserId.eq(user_id))
            .all(&self.database)
            .await?;
        let mut team_ids = vec![];
        for team in teams {
            team_ids.push(team.team_id);
        }
        let team = teams::Entity::find()
            .filter(teams::Column::Uid.is_in(team_ids))
            .all(&self.database)
            .await;
        match team {
            Ok(team) => Ok(team),
            Err(e) => Err(JZError::Other(anyhow!("[68] {}", e.to_string()))),
        }
    }
}
