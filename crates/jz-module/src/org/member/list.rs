use uuid::Uuid;
use jz_model::{member, organization, repository, users};
use crate::AppModule;
use sea_orm::*;
use serde_json::json;

impl AppModule {
    pub async fn member_list_by_name(&self, org: String) -> anyhow::Result<Vec<users::Model>> {
        let org = organization::Entity::find()
            .filter(organization::Column::Name.eq(org))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("org not found"))?;
        let members = member::Entity::find()
            .filter(member::Column::GroupUid.eq(org.uid))
            .all(&self.read)
            .await?;
        let mut result = Vec::new();
        for member in members {
            let user = users::Entity::find()
                .filter(users::Column::Uid.eq(member.users_uid))
                .one(&self.read)
                .await?
                .ok_or(anyhow::anyhow!("user not found"))?;
            result.push(user);
        }
        Ok(result)
    }
    
    pub async fn member_list_by_name_values(&self, org: String) -> anyhow::Result<Vec<serde_json::Value>> {
        let org = organization::Entity::find()
            .filter(organization::Column::Name.eq(org))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("org not found"))?;
        let members = member::Entity::find()
            .filter(member::Column::GroupUid.eq(org.uid))
            .all(&self.read)
            .await?;
        let mut result = Vec::new();
        for member in members {
            let user = users::Entity::find()
                .filter(users::Column::Uid.eq(member.users_uid))
                .one(&self.read)
                .await?
                .ok_or(anyhow::anyhow!("user not found"))?;
            result.push(json!({
                "uid": user.uid,
                "username": user.username,
                "email": user.email,
                "access": member.access,
                "join_at": member.join_at,
            }));
        }
        Ok(result)
    }
    pub async fn member_list_by_uid(&self, orguid: Uuid) -> anyhow::Result<Vec<users::Model>> {
        let _ = organization::Entity::find()
            .filter(organization::Column::Uid.eq(orguid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("org not found"))?;
        let members = member::Entity::find()
            .filter(member::Column::GroupUid.eq(orguid))
            .all(&self.read)
            .await?;
        let mut result = Vec::new();
        for member in members {
            let user = users::Entity::find()
                .filter(users::Column::Uid.eq(member.users_uid))
                .one(&self.read)
                .await?
                .ok_or(anyhow::anyhow!("user not found"))?;
            result.push(user);
        }
        Ok(result)
    }
    pub async fn member_orgs(&self, users_uid: Uuid) -> anyhow::Result<Vec<organization::Model>> {
        let member = member::Entity::find()
            .filter(member::Column::UsersUid.eq(users_uid))
            .all(&self.read)
            .await?;
        let mut result = vec![];
        for idx in member {
            let org = organization::Entity::find()
                .filter(organization::Column::Uid.eq(idx.group_uid))
                .one(&self.read)
                .await?
                .ok_or(anyhow::anyhow!("org not found"))?;
            result.push(org);
        }
        Ok(result)
    }
    pub async fn member_owner_access(&self, users_uid: Uuid) -> anyhow::Result<Vec<serde_json::Value>> {
        let member = member::Entity::find()
            .filter(member::Column::UsersUid.eq(users_uid))
            .all(&self.read)
            .await?;
        let mut result = vec![];
        for idx in member {
            let org = organization::Entity::find()
                .filter(organization::Column::Uid.eq(idx.group_uid))
                .one(&self.read)
                .await?
                .ok_or(anyhow::anyhow!("org not found"))?;
            let member = member::Entity::find()
                .filter(member::Column::GroupUid.eq(org.uid))
                .filter(member::Column::UsersUid.eq(users_uid))
                .one(&self.read)
                .await?
                .ok_or(anyhow::anyhow!("member not found"))?;
            let nums_member = member::Entity::find()
                .filter(member::Column::GroupUid.eq(org.uid))
                .count(&self.read)
                .await?;
            let nums_repo = repository::Entity::find()
                .filter(repository::Column::OwnerUid.eq(org.uid))
                .count(&self.read)
                .await?;
            result.push(json!({
                "org": org,
                "member": member,
                "nums_member": nums_member,
                "nums_repo": nums_repo,
                "access": member.access,
            }));
        }
        Ok(result)
    }
    pub async fn member_can_setting(&self, org: String, users_uid: Uuid) -> anyhow::Result<bool> {
        let org = organization::Entity::find()
            .filter(organization::Column::Name.eq(org))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("org not found"))?;
        let member = member::Entity::find()
            .filter(member::Column::UsersUid.eq(users_uid))
            .filter(member::Column::GroupUid.eq(org.uid))
            .one(&self.read)
            .await?
            .ok_or(anyhow::anyhow!("member not found"))?;
        Ok(member.access > 50)
    }
}