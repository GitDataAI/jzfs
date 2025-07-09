use std::collections::HashSet;
use sea_orm::*;
use sea_orm::prelude::Expr;
use uuid::Uuid;
use authd::users;
use cert::schema::{result_error_with_msg, result_ok, AppResult};
use crate::schema::users::UserBasicFromParam;
use crate::service::AppWorkHorse;

impl AppWorkHorse {
    pub async fn service_user_basic_from(&self, user_uid: Uuid, param: UserBasicFromParam) -> AppResult<()> {
        let user = users::Entity::find_by_id(user_uid)
            .one(&self.db)
            .await;
        let Ok(Some(user)) = user else { 
            return result_error_with_msg("User Nod Found".to_string())
        };
        let mut active = user.clone().into_active_model();
        if let Some( description) = param.description {
            active.description = Set(Some(description));
        }
        if !param.website.is_empty() {
            let mut set = HashSet::new();
            let _ = user.website.iter().map(|x| set.insert(x.clone()));
            let _ = param.website.iter().map(|x| set.insert(x.clone()));
            active.website = Set(set.into_iter().collect::<Vec<String>>());
        }
        if let Some(timezone) = param.timezone {
            active.timezone = Set(Some(timezone));
        }
        if let Some(language) = param.language {
            active.language = Set(Some(language));
        }
        if let Some(location) = param.location {
            active.location = Set(Some(location));
        }
        match active.update(&self.db).await {
            Ok(_) => {
                result_ok()
            }
            Err(error) => {
                result_error_with_msg(error.to_string())
            }
        }
    }
    pub async fn service_user_update_avatar(&self, user_uid: Uuid, avatar: String) -> AppResult<()> {
        match users::Entity::update_many()
            .col_expr(users::Column::Avatar, Expr::value(Some(avatar)))
            .filter(users::Column::Uid.eq(user_uid))
            .exec(&self.db)
            .await {
            Ok(_) => {
                result_ok()
            }
            Err(error) => {
                result_error_with_msg(error.to_string())
            }
        }
    }
    pub async fn service_user_clear_avatar(&self, user_uid: Uuid) -> AppResult<()> {
        match users::Entity::update_many()
            .col_expr(users::Column::Avatar, Expr::value(None::<String>))
            .filter(users::Column::Uid.eq(user_uid))
            .exec(&self.db)
            .await {
            Ok(_) => {
                result_ok()
            }
            Err(error) => {
                result_error_with_msg(error.to_string())
            }
        }
    }
}