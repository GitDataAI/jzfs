use sea_orm::{Condition, IntoActiveModel, QueryFilter, QueryOrder, QuerySelect};
use sea_orm::ColumnTrait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use jz_model::{issues, tags, uuid_v7};
use crate::AppModule;

#[derive(Deserialize)]
pub struct AddIssues {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize,Serialize)]
pub struct ListParam {
    pub page: u64,
    pub size: u64,
    pub state: Option<String>,
    pub tags: Option<Vec<Uuid>>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_by: Option<Uuid>
}

impl AppModule {
    pub async fn issue_add(
        &self,
        owner: String,
        repo: String,
        param: AddIssues,
        opsuid: Uuid
    ) -> anyhow::Result<()> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let issues = issues::ActiveModel{
            uid: Set(uuid_v7()),
            repo_uid: Set(repo.uid),
            title: Set(param.title),
            body: Set(param.body),
            state: Set("open".to_string()),
            tags: Set(vec![]),
            closed_at: Set(None),
            closed_by: Set(None),
            created_at: Set(Utc::now().naive_utc()),
            created_by: Set(opsuid),
            updated_at: Set(Utc::now().naive_utc()),
            id: Default::default(),
        };
        issues.insert(&self.write).await?;
        Ok(())
    }
    pub async fn issue_list(
        &self,
        owner: String,
        repo: String,
    ) -> anyhow::Result<Vec<issues::Model>> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let issues = issues::Entity::find()
            .filter(issues::Column::RepoUid.eq(repo.uid))
            .all(&self.read)
            .await?;
        Ok(issues)
    }
    
    pub async fn issues_lists(
        &self,
        owner: String,
        repo: String,
        param: ListParam,
    ) -> anyhow::Result<Vec<issues::Model>> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let mut condition = Condition::any();
        if let Some(state) = param.state {
            condition = condition.add(issues::Column::State.eq(state));
        }
        if let Some(title) = param.title {
            condition = condition.add(issues::Column::Title.contains(title));
        };
        if let Some(body) = param.body {
            condition = condition.add(issues::Column::Body.contains(body));
        };
        if let Some(tags) = param.tags {
            condition = condition.add(issues::Column::Tags.is_in(tags));
        }
        let issues = issues::Entity::find()
            .filter(condition)
            .filter(issues::Column::RepoUid.eq(repo.uid))
            .order_by_desc(issues::Column::Id)
            .limit(param.size)
            .offset(param.page * param.size)
            .all(&self.read)
            .await?;
        Ok(issues)
    }
    pub async fn issues_add_tags(
        &self,
        owner: String,
        repo: String,
        issue_uid: Uuid,
        tags_uid: Uuid,
    ) -> anyhow::Result<()> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let tags = tags::Entity::find()
            .filter(tags::Column::RepoUid.eq(repo.uid))
            .filter(tags::Column::Uid.eq(tags_uid))
            .one(&self.read)
            .await?;
        if let None = tags {
            return Err(anyhow::anyhow!("tag not found"));
        }
        let issues = issues::Entity::find()
            .filter(issues::Column::RepoUid.eq(repo.uid))
            .filter(issues::Column::Uid.eq(issue_uid))
            .one(&self.read)
            .await?;
        if let None = issues {
            return Err(anyhow::anyhow!("issue not found"));
        }
        let mut issues = issues.unwrap().into_active_model();
        let tags = issues.tags.clone().unwrap();
        if tags.contains(&tags_uid) {
            return Err(anyhow::anyhow!("tag already exists"));
        }
        issues.tags = Set(tags.into_iter().chain(vec![tags_uid]).collect());
        issues.update(&self.write).await?;
        Ok(())
    }
    pub async fn issues_del_tags(
        &self,
        owner: String,
        repo: String,
        issue_uid: Uuid,
        tags_uid: Uuid,
    ) -> anyhow::Result<()> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let tags = tags::Entity::find()
            .filter(tags::Column::RepoUid.eq(repo.uid))
            .filter(tags::Column::Uid.eq(tags_uid))
            .one(&self.read)
            .await?;
        if let None = tags {
            return Err(anyhow::anyhow!("tag not found"));
        }
        let issues = issues::Entity::find()
            .filter(issues::Column::RepoUid.eq(repo.uid))
            .filter(issues::Column::Uid.eq(issue_uid))
            .one(&self.read)
            .await?;
        if let None = issues {
            return Err(anyhow::anyhow!("issue not found"));
        }
        let mut issues = issues.unwrap().into_active_model();
        let tags = issues.tags.clone().unwrap();
        if !tags.contains(&tags_uid) {
            return Err(anyhow::anyhow!("tag not exists"));
        }
        issues.tags = Set(tags.into_iter().filter(|x| *x != tags_uid).collect());
        issues.update(&self.write).await?;
        Ok(())
    }
    pub async fn issues_info_by_id(&self, is: i32, repo_uid: Uuid) -> anyhow::Result<issues::Model> {
        let issues = issues::Entity::find()
            .filter(issues::Column::RepoUid.eq(repo_uid))
            .filter(issues::Column::Id.eq(is))
            .one(&self.read)
            .await?;
        if let None = issues {
            return Err(anyhow::anyhow!("issue not found"));
        }
        Ok(issues.unwrap())
    }
}