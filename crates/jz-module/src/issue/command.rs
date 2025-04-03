use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;
use sea_orm::*;
use jz_model::{comments, uuid_v7};
use crate::AppModule;

#[derive(Deserialize)]
pub struct AddCommand {
    pub issues_uid: Uuid,
    pub issues_id: i32,
    pub command: String,
    pub parent: Option<Uuid>,
}



impl AppModule {
    pub async fn comments_add(
        &self,
        owner: String,
        repo: String,
        issues_id: i32,
        param: AddCommand,
        ops_uid: Uuid,
    ) -> anyhow::Result<()> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let issues = self.issues_info_by_id(issues_id,repo.uid).await?;
        let command = comments::ActiveModel{
            uid: Set(uuid_v7()),
            body: Set(param.command),
            issue_uid: Set(issues.uid),
            parent: Set(param.parent),
            created_by: Set(ops_uid),
            created_at: Set(Utc::now().naive_utc()),
        };
        command.insert(&self.write).await?;
        Ok(())
    }
    pub async fn comments_list(
        &self,
        owner: String,
        repo: String,
        issues_id: i32,
    ) -> anyhow::Result<Vec<comments::Model>> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let issues = self.issues_info_by_id(issues_id,repo.uid).await?;
        let commands = comments::Entity::find()
            .filter(comments::Column::IssueUid.eq(issues.uid))
            .all(&self.read)
            .await?;
        Ok(commands)
    }
    pub async fn comments_edit(
        &self,
        owner: String,
        repo: String,
        issues_id: i32,
        command_uid: Uuid,
        param: AddCommand,
        ops_uid: Uuid,
    ) -> anyhow::Result<()> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let issues = self.issues_info_by_id(issues_id,repo.uid).await?;
        let command = comments::Entity::find_by_id(command_uid)
            .one(&self.read)
            .await?;
        if let Some(command) = command {
            if command.created_by != ops_uid {
                return Err(anyhow::anyhow!("permission denied"));
            }
            let command = comments::ActiveModel {
                uid: Set(command_uid),
                body: Set(param.command),
                issue_uid: Set(issues.uid),
                parent: Set(param.parent),
                created_by: Set(ops_uid),
                created_at: Set(Utc::now().naive_utc()),
            };
            command.update(&self.write).await?;
        }
        Ok(())
    }
    pub async fn comments_del(
        &self,
        owner: String,
        repo: String,
        issues_id: i32,
        command_uid: Uuid,
        ops_uid: Uuid,
    ) -> anyhow::Result<()> {
        let repo = self.repo_info_by_owner_and_name(owner, repo).await?;
        let _issues = self.issues_info_by_id(issues_id,repo.uid).await?;
        let command = comments::Entity::find_by_id(command_uid)
            .one(&self.read)
            .await?;
        if let Some(command) = command {
            if command.created_by != ops_uid {
                return Err(anyhow::anyhow!("permission denied"));
            }
            command.delete(&self.write).await?;
        }
        Ok(())
    }
}