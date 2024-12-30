use crate::api::handlers::repos::RepoCreateOwnerList;
use crate::error::{JZError, JZResult};
use crate::models::groups::groups;
use crate::models::teams::{teams, teamsus};
use crate::models::users::users;
use crate::server::MetaData;
use sea_orm::*;
use uuid::Uuid;

impl MetaData {
    pub async fn repo_groups_teams_access(
        &self,
        owner: String,
        name: String,
        user_id: Uuid,
    ) -> JZResult<i64> {
        let info = match self.repo_info(owner, name).await {
            Ok(info) => info,
            Err(_e) => return Ok(0),
        };
        if info.private {
            return Ok(0);
        }
        if info.is_group {
            let result = groups::Entity::find()
                .filter(groups::Column::Uid.eq(info.uid))
                .one(&self.database)
                .await?;
            if result.is_none() {
                return Ok(0);
            }
            let teams = match teams::Entity::find()
                .filter(teams::Column::OrgId.eq(result.unwrap().uid))
                .all(&self.database)
                .await
            {
                Ok(teams) => teams,
                Err(_) => return Ok(1),
            };
            if teams.is_empty() {
                return Ok(1);
            }
            for team in teams {
                let members = match teamsus::Entity::find()
                    .filter(teamsus::Column::TeamId.eq(team.uid))
                    .all(&self.database)
                    .await
                {
                    Ok(members) => members,
                    Err(_) => return Ok(1),
                };
                for member in members {
                    if member.uid == user_id {
                        return Ok(member.access);
                    }
                }
            }
            Err(JZError::Other(anyhow::anyhow!(
                "[042] Team Members Not Found"
            )))
        } else {
            Err(JZError::Other(anyhow::anyhow!("[038] Repo Not Group")))
        }
    }
    pub async fn repo_owner_list_check(
        &self,
        user_id: Uuid,
    ) -> anyhow::Result<Vec<RepoCreateOwnerList>> {
        let teams = teamsus::Entity::find()
            .filter(teamsus::Column::UserId.eq(user_id))
            .all(&self.database)
            .await?
            .iter()
            .map(|x| x.team_id)
            .collect::<Vec<_>>();
        let teams = teams::Entity::find()
            .filter(teams::Column::Uid.is_in(teams))
            .all(&self.database)
            .await?;
        let mut results = Vec::<RepoCreateOwnerList>::new();
        for group_id in teams {
            let members = groups::Entity::find()
                .filter(groups::Column::Uid.eq(group_id.org_id.clone()))
                .all(&self.database)
                .await?;
            for member in members {
                let avatar_url = member.avatar_url.clone().unwrap_or_else(|| "".to_string());
                let result = RepoCreateOwnerList {
                    uid: group_id.clone().uid,
                    name: group_id.clone().name,
                    group: member.name,
                    avatar_url,
                };
                results.push(result);
                break;
            }
        }
        let owner = users::Entity::find()
            .filter(users::Column::Uid.eq(user_id))
            .one(&self.database)
            .await?;
        if owner.is_none() {
            return Ok(results);
        }
        let owner = owner.unwrap();
        results.push(RepoCreateOwnerList {
            name: owner.username,
            uid: owner.uid,
            group: "".to_string(),
            avatar_url: owner.avatar_url.unwrap_or_else(|| "".to_string()),
        });
        Ok(results)
    }
}
