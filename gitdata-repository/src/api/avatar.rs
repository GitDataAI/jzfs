use actix_session::Session;
use actix_web::Responder;
use actix_web::web;
use lib_entity::session::USER_SESSION_KEY;
use lib_entity::session::UsersSessionModel;
use lib_entity::write::AppWrite;

use crate::service::AppFsState;
use crate::service::avatar::AvatarUploadParma;

pub async fn update_avatar(
    state : web::Data<AppFsState>,
    path : web::Path<(String, String)>,
    web::Json(avatar_url) : web::Json<AvatarUploadParma>,
    session : Session,
) -> impl Responder {
    let user = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(user)) => user,
        _ => return AppWrite::unauthorized("unauthorized".to_string()),
    };
    let repository = match state.info_to_uid(path.0.clone(), path.1.clone()).await {
        Ok(repository) => repository,
        Err(_) => return AppWrite::not_found("repository not found".to_string()),
    };
    match state
        .avatar_update(repository, user.uid, avatar_url.avatar_url)
        .await
    {
        Ok(_) => AppWrite::ok(()),
        Err(e) => AppWrite::error(e.to_string()),
    }
}

pub async fn delete_avatar(
    state : web::Data<AppFsState>,
    path : web::Path<(String, String)>,
    session : Session,
) -> impl Responder {
    let user = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(user)) => user,
        _ => return AppWrite::unauthorized("unauthorized".to_string()),
    };
    let repository = match state.info_to_uid(path.0.clone(), path.1.clone()).await {
        Ok(repository) => repository,
        Err(_) => return AppWrite::not_found("repository not found".to_string()),
    };
    match state.avatar_delete(repository, user.uid).await {
        Ok(_) => AppWrite::ok(()),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
