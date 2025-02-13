use lib_entity::ColumnTrait;
use lib_entity::EntityTrait;
use lib_entity::QueryFilter;
use lib_entity::repos::repos;
use lib_entity::users::users;
use serde::Serialize;
use uuid::Uuid;

use crate::service::AppFsState;

#[derive(Serialize)]
pub struct RepositoryInfo {
    pub uid : Uuid,
    pub owner : users::Model,
    pub avatar_url : Option<String>,
    pub name : String,
    pub description : Option<String>,
    pub website : Option<String>,
    pub private : bool,
    pub is_group : bool,
    pub has_issues : bool,
    pub has_idcard : bool,
    pub has_wiki : bool,
    pub has_downloads : bool,
    pub has_projects : bool,
    pub topic : Vec<String>,
    pub default_branchs : Option<String>,
    pub nums_star : i64,
    pub nums_fork : i64,
    pub nums_watcher : i64,
    pub nums_commit : i64,
    pub nums_release : i64,
    pub nums_tag : i64,
    pub nums_branchs : i64,
    pub nums_members : i64,
    pub fork : bool,
    pub fork_from : Option<repos::Model>,
    pub created : i64,
    pub updated : i64,
    pub node : Uuid,
    pub collaborators : Vec<users::Model>,
}

impl AppFsState {
    pub async fn info(&self, owner : String, repo : String) -> anyhow::Result<RepositoryInfo> {
        let owner = users::Entity::find()
            .filter(users::Column::Username.eq(owner.clone()))
            .one(&self.read)
            .await
            .map_err(|err| anyhow::anyhow!("{}", err))?
            .ok_or(anyhow::anyhow!("user not found"))?;
        let rspo = repos::Entity::find()
            .filter(repos::Column::OwnerId.eq(owner.uid))
            .filter(repos::Column::Name.eq(repo))
            .one(&self.read)
            .await
            .map_err(|err| anyhow::anyhow!("{}", err))?
            .ok_or(anyhow::anyhow!("repo not found"))?;
        let member = users::Entity::find()
            .filter(users::Column::Uid.is_in(rspo.collaborators))
            .all(&self.read)
            .await
            .map_err(|err| anyhow::anyhow!("{}", err))?;
        let result = RepositoryInfo {
            uid : rspo.uid,
            owner,
            avatar_url : rspo.avatar_url,
            name : rspo.name,
            description : rspo.description,
            website : rspo.website,
            private : rspo.private,
            is_group : rspo.is_group,
            has_issues : rspo.has_issues,
            has_idcard : rspo.has_idcard,
            has_wiki : rspo.has_wiki,
            has_downloads : rspo.has_downloads,
            has_projects : rspo.has_projects,
            topic : rspo.topic,
            default_branchs : None,
            nums_star : rspo.nums_star,
            nums_fork : rspo.nums_fork,
            nums_watcher : rspo.nums_watcher,
            nums_commit : rspo.nums_commit,
            nums_release : rspo.nums_release,
            nums_tag : rspo.nums_tag,
            nums_branchs : rspo.nums_branchs,
            nums_members : rspo.nums_members,
            fork : rspo.fork,
            fork_from : None,
            created : rspo.created,
            updated : rspo.updated,
            node : rspo.node,
            collaborators : member,
        };
        Ok(result)
    }
    pub async fn info_to_uid(&self, owner : String, repo : String) -> anyhow::Result<Uuid> {
        let owner = users::Entity::find()
            .filter(users::Column::Username.eq(owner.clone()))
            .one(&self.read)
            .await
            .map_err(|err| anyhow::anyhow!("{}", err))?
            .ok_or(anyhow::anyhow!("user not found"))?;
        let rspo = repos::Entity::find()
            .filter(repos::Column::OwnerId.eq(owner.uid))
            .filter(repos::Column::Name.eq(repo))
            .one(&self.read)
            .await
            .map_err(|err| anyhow::anyhow!("{}", err))?
            .ok_or(anyhow::anyhow!("repo not found"))?;
        Ok(rspo.uid)
    }
}
