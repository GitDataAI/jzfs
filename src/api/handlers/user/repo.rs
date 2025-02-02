use crate::api::app_writer::AppWrite;
use crate::api::middleware::session::SessionModel;
use crate::models::repos::repos;
use crate::models::users::users::UpdateOption;
use crate::server::MetaData;
use actix_session::Session;
use actix_web::{Responder, web};
use std::collections::HashMap;

pub async fn user_option(
    session: Session,
    meta: web::Data<MetaData>,
    option: web::Json<UpdateOption>,
) -> impl Responder {
    let model = match SessionModel::authenticate(session.clone()).await {
        Ok(model) => model,
        Err(err) => return AppWrite::<Option<String>>::unauthorized(err.to_string()),
    };
    match meta
        .users_update_option(model.uid, option.into_inner())
        .await
    {
        Ok(_) => {
            model.sync(session, meta.clone().into_inner()).await.ok();
            AppWrite::ok(None)
        }
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

pub async fn users_repos(
    session: Session,
    meta: web::Data<MetaData>,
    option: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let uid = if let Some(username) = option.get("username") {
        match meta.users_info_username(username.clone()).await {
            Ok(user) => user.uid,
            Err(_) => {
                return AppWrite::<Vec<repos::Model>>::fail("[001] User Not Found".to_string());
            }
        }
    } else {
        match SessionModel::authenticate(session).await {
            Ok(model) => model.uid,
            Err(err) => return AppWrite::<Vec<repos::Model>>::unauthorized(err.to_string()),
        }
    };
    match meta.users_repo_list(uid).await {
        Ok(repos) => AppWrite::ok(repos),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
