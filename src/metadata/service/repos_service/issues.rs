use std::str::FromStr;
use futures_util::TryStreamExt;
use mongodb::bson::doc;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::api::middleware::session::model::SessionModel;
use crate::metadata::model::users::users_data;
use crate::metadata::mongo::issues::{IssuesAssignees, IssuesComment, IssuesLabels, IssuesModel};
use crate::metadata::service::repos_service::RepoService;
use sea_orm::*;
use sea_orm::prelude::Expr;
use crate::metadata::model::repo::repo;

impl RepoService {
    pub async fn issues(&self, owner: String, repo: String, page: u64, size: i64) -> anyhow::Result<Vec<IssuesModel>>{
        let repo_id = self.owner_name_by_model(owner, repo).await;
        if repo_id.is_err(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        let repo_id = repo_id?.uid.to_string();
        let mut issues = self.mongo.issues.find(
            doc! {
                "repo_id": repo_id
            }
        )
            .skip(page * (size as u64))
            .limit(size)
            .await?;
        let mut result = vec![];
        while let Some(issue) = issues.try_next().await? {
            result.push(issue);
        }
        Ok(result)
    }
    pub async fn issues_detail(&self, issues_id: Uuid) -> anyhow::Result<IssuesModel>{
        let issues = self.mongo.issues.find_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        if issues.is_none(){
            return Err(anyhow::anyhow!("issues not found"))
        }
        Ok(issues.unwrap())
    }
    pub async fn issues_count(&self, owner: String, repo: String) -> anyhow::Result<u64>{
        let repo_id = self.owner_name_by_model(owner, repo).await;
        if repo_id.is_err(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        let repo_id = repo_id?.uid.to_string();
        let count = self.mongo.issues.count_documents(
            doc! {
                "repo_id": repo_id
            }
        )
            .await?;
        Ok(count)
    }
    pub async fn issues_create(&self, owner: String, repo: String, title: String, body: String, created_by: SessionModel) -> anyhow::Result<IssuesModel>{
        let repo = self.owner_name_by_model(owner, repo).await;
        if repo.is_err(){
            return Err(anyhow::anyhow!("repo not found"))
        }
        let repo_id = repo?.uid.to_string();
        let issues_id = Uuid::new_v4();
        let issues = IssuesModel{
            issues_id,
            repo_id: repo_id.clone(),
            title,
            body,
            created_by: created_by.uid,
            created_username: created_by.username,
            assignees: vec![],
            labels: vec![],
            comments: vec![],
            created_at: Some(OffsetDateTime::now_utc().unix_timestamp()),
            updated_at: None,
            closed_at: None,
            closed: false,
            notifications: vec![],
            participant: vec![],
        };
        self.mongo.issues.insert_one(issues.clone()).await?;
        users_data::Entity::update_many()
            .filter(users_data::Column::UserId.eq(created_by.uid))
            .col_expr(
                users_data::Column::Issue,
                Expr::col(users_data::Column::Issue).add(issues_id)
            )
            .exec(&self.db)
            .await?;
        repo::Entity::update_many()
            .filter(repo::Column::Uid.eq(Uuid::from_str(&repo_id).unwrap()))
            .col_expr(
                repo::Column::Issue,
                Expr::col(repo::Column::Issue).add(1)
            )
            .col_expr(
                repo::Column::OpenIssue,
                Expr::col(repo::Column::OpenIssue).add(1)
            )
            .exec(&self.db)
            .await?;
        Ok(issues)
    }
    pub async fn issues_comment(&self, issues_id: Uuid, session_model: SessionModel, body: String, avatar: Option<String>, role: String) -> anyhow::Result<IssuesModel>{
        let issues = self.mongo.issues.find_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        if issues.is_none(){
            return Err(anyhow::anyhow!("issues not found"))
        }
        let mut issues = issues.unwrap();
        issues.comments.push(IssuesComment{
            comment_id: Uuid::new_v4(),
            user_id: session_model.uid,
            name: session_model.username,
            avatar,
            body,
            role,
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            reply: None,
        });
        self.mongo.issues.delete_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        self.mongo.issues.insert_one(issues.clone()).await?;
        Ok(issues)
    }
    pub async fn issues_comment_reply(&self, issues_id: Uuid, session_model: SessionModel,commend_id: Uuid ,body: String, avatar: Option<String>, role: String) -> anyhow::Result<()>{
        let issues = self.mongo.issues.find_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        if issues.is_none(){
            return Err(anyhow::anyhow!("issues not found"))
        }
        let mut issues = issues.unwrap();
        let mut comments = issues.comments.clone();
        for (_, comment) in comments.iter_mut().enumerate(){
            if comment.comment_id == commend_id {
                if comment.reply.is_none(){
                    comment.reply = Some(Box::new(vec![]));
                }
                comment.reply.as_mut().unwrap().push(IssuesComment{
                    comment_id: Uuid::new_v4(),
                    user_id: session_model.uid.clone(),
                    name: session_model.username.clone(),
                    avatar,
                    body,
                    role,
                    created_at: OffsetDateTime::now_utc().unix_timestamp(),
                    reply: None,
                });
                break;
            }
        }
        self.mongo.issues.delete_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        issues.comments = comments;
        self.mongo.issues.insert_one(issues.clone()).await?;
        Ok(())
    }
    pub async fn issues_add_label(&self, issues_id: Uuid, label: IssuesLabels) -> anyhow::Result<()>{
        let issues = self.mongo.issues.find_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        if issues.is_none(){
            return Err(anyhow::anyhow!("issues not found"))
        }
        let mut issues = issues.unwrap();
        issues.labels.push(label);
        self.mongo.issues.delete_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        self.mongo.issues.insert_one(issues.clone()).await?;
        Ok(())
    }
    pub async fn issues_remove_label(&self, issues_id: Uuid, label: IssuesLabels) -> anyhow::Result<()>{
        let issues = self.mongo.issues.find_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        if issues.is_none(){
            return Err(anyhow::anyhow!("issues not found"))
        }
        let mut issues = issues.unwrap();
        issues.labels.retain(|x| x.name != label.name);
        self.mongo.issues.delete_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        self.mongo.issues.insert_one(issues.clone()).await?;
        Ok(())
    }
    pub async fn issues_add_assignees(&self, issues_id: Uuid, assignees: IssuesAssignees) -> anyhow::Result<()>{
        let issues = self.mongo.issues.find_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        if issues.is_none(){
            return Err(anyhow::anyhow!("issues not found"))
        }
        let mut issues = issues.unwrap();
        issues.assignees.push(assignees);
        self.mongo.issues.delete_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        self.mongo.issues.insert_one(issues.clone()).await?;
        Ok(())
    }
    pub async fn issues_remove_assignees(&self, issues_id: Uuid, assignees: IssuesAssignees) -> anyhow::Result<()>{
        let issues = self.mongo.issues.find_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        if issues.is_none(){
            return Err(anyhow::anyhow!("issues not found"))
        }
        let mut issues = issues.unwrap();
        issues.assignees.retain(|x| x.user_id != assignees.user_id);
        self.mongo.issues.delete_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        self.mongo.issues.insert_one(issues.clone()).await?;
        Ok(())
    }
    pub async fn issues_add_participant(&self, issues_id: Uuid, participant: IssuesAssignees) -> anyhow::Result<()>{
        let issues = self.mongo.issues.find_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        if issues.is_none(){
            return Err(anyhow::anyhow!("issues not found"))
        }
        let mut issues = issues.unwrap();
        issues.participant.push(participant);
        self.mongo.issues.delete_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        self.mongo.issues.insert_one(issues.clone()).await?;
        Ok(())
    }
    pub async fn issues_close(&self, issues_id: Uuid) -> anyhow::Result<()>{
        let issues = self.mongo.issues.find_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        if issues.is_none(){
            return Err(anyhow::anyhow!("issues not found"))
        }
        let mut issues = issues.unwrap();
        issues.closed = true;
        self.mongo.issues.delete_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        self.mongo.issues.insert_one(issues.clone()).await?;
        repo::Entity::update_many()
            .filter(repo::Column::Uid.eq(issues.repo_id))
            .col_expr(repo::Column::OpenIssue, Expr::col(repo::Column::Issue).div(1))
            .col_expr(repo::Column::CloseIssue, Expr::col(repo::Column::Issue).add(1))
            .exec(&self.db)
            .await?;
        Ok(())
    }
    pub async fn issues_open(&self, issues_id: Uuid) -> anyhow::Result<()>{
        let issues = self.mongo.issues.find_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        if issues.is_none(){
            return Err(anyhow::anyhow!("issues not found"))
        }
        let mut issues = issues.unwrap();
        issues.closed = false;
        self.mongo.issues.delete_one(
            doc! {
                "issues_id": issues_id.to_string()
            }
        )
            .await?;
        self.mongo.issues.insert_one(issues.clone()).await?;
        repo::Entity::update_many()
            .filter(repo::Column::Uid.eq(issues.repo_id))
            .col_expr(repo::Column::OpenIssue, Expr::col(repo::Column::Issue).add(1))
            .col_expr(repo::Column::CloseIssue, Expr::col(repo::Column::Issue).sub(1))
            .exec(&self.db)
            .await?;
        Ok(())
    }
    
}