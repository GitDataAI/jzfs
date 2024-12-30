use crate::error::{JZError, JZResult};
use crate::models::repos::repos;
use crate::models::teams::{teams, teamsus};
use crate::models::users::users;
use crate::server::MetaData;
use anyhow::anyhow;
use sea_orm::prelude::Expr;
use sea_orm::*;
use sha256::Sha256Digest;
use uuid::Uuid;

impl MetaData {
    pub async fn users_update_avatar(&self, uid: Uuid, avatar_url: String) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[53] User Not Found")));
        }
        let result = users::Entity::update_many()
            .filter(users::Column::Uid.eq(uid))
            .col_expr(users::Column::AvatarUrl, Expr::value(avatar_url))
            .col_expr(
                users::Column::Updated,
                Expr::value(chrono::Local::now().timestamp()),
            )
            .exec(&self.database)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(JZError::Other(anyhow!("[54] {:?}", err))),
        }
    }
    pub async fn users_team_list(&self, uid: Uuid) -> JZResult<Vec<teams::Model>> {
        let teams = teamsus::Entity::find()
            .filter(teamsus::Column::UserId.eq(uid))
            .all(&self.database)
            .await?;
        let mut models = Vec::new();
        for team in teams {
            let model = teams::Entity::find_by_id(team.team_id)
                .one(&self.database)
                .await?;
            if model.is_none() {
                continue;
            }
            models.push(model.unwrap());
        }
        Ok(models)
    }
    pub async fn users_repo_list(&self, uid: Uuid) -> JZResult<Vec<repos::Model>> {
        let models = repos::Entity::find()
            .filter(repos::Column::OwnerId.eq(uid))
            .all(&self.database)
            .await?;
        Ok(models)
    }
    pub async fn users_update_option(
        &self,
        uid: Uuid,
        option: users::UpdateOption,
    ) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[49] User Not Found")));
        }
        let mut result = users::Entity::update_many().filter(users::Column::Uid.eq(uid));
        if let Some(name) = option.name {
            result = result.col_expr(users::Column::Name, Expr::value(name));
        }
        if let Some(bio) = option.bio {
            result = result.col_expr(users::Column::Bio, Expr::value(bio));
        }
        if let Some(pronouns) = option.pronouns {
            result = result.col_expr(users::Column::Pronouns, Expr::value(pronouns));
        }
        if let Some(company) = option.company {
            result = result.col_expr(users::Column::Company, Expr::value(company));
        }
        if let Some(location) = option.location {
            result = result.col_expr(users::Column::Location, Expr::value(location));
        }
        if let Some(localtime) = option.localtime {
            result = result.col_expr(users::Column::Localtime, Expr::value(localtime));
        }
        if let Some(i18n) = option.i18n {
            result = result.col_expr(users::Column::I18n, Expr::value(i18n));
        }
        if let Some(website) = option.website {
            result = result.col_expr(users::Column::Website, Expr::value(website));
        }
        if let Some(orcid) = option.orcid {
            result = result.col_expr(users::Column::Orcid, Expr::value(orcid));
        }
        if let Some(social) = option.social {
            result = result.col_expr(users::Column::Social, Expr::value(social));
        }
        if let Some(theme) = option.theme {
            result = result.col_expr(users::Column::Theme, Expr::value(theme));
        }
        if let Some(pinned) = option.pinned {
            result = result.col_expr(users::Column::Pinned, Expr::value(pinned));
        }
        result = result.col_expr(
            users::Column::Updated,
            Expr::value(chrono::Local::now().timestamp()),
        );
        let result = result.exec(&self.database).await;
        match result {
            Ok(x) => {
                dbg!(x);
                Ok(())
            }
            Err(err) => Err(JZError::Other(anyhow!("[50] {:?}", err))),
        }
    }
    pub async fn users_update_password(&self, uid: Uuid, password: String) -> JZResult<()> {
        if !self.check_users_id(uid).await? {
            return Err(JZError::Other(anyhow!("[51] User Not Found")));
        }
        let result = users::Entity::update_many()
            .filter(users::Column::Uid.eq(uid))
            .col_expr(users::Column::Password, Expr::value(password.digest()))
            .col_expr(
                users::Column::Updated,
                Expr::value(chrono::Local::now().timestamp()),
            )
            .exec(&self.database)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(JZError::Other(anyhow!("[52] {:?}", err))),
        }
    }
}
