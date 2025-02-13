use actix_session::Session;
use actix_web::Responder;
use actix_web::web;
use lib_entity::session::USER_SESSION_KEY;
use lib_entity::session::UsersSessionModel;
use lib_entity::write::AppWrite;

use crate::service::AppFsState;

pub async fn delete_branch(
    state : web::Data<AppFsState>,
    path : web::Path<(String, String, String)>,
    session : Session,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("Not Login".to_string()),
    };
    let repo = match state
        .info_to_uid(path.0.to_string(), path.1.to_string())
        .await
    {
        Ok(repo) => repo,
        Err(err) => return AppWrite::bad_request(err.to_string()),
    };
    match state.access(users.uid, repo.clone()).await {
        Ok(x) => {
            if !x {
                return AppWrite::forbidden("No Permission".to_string());
            }
        }
        Err(err) => return AppWrite::bad_request(err.to_string()),
    }
    match state.delete_branch(repo, path.2.to_string()).await {
        Ok(_) => AppWrite::<String>::ok("Success".to_string()),
        Err(err) => AppWrite::bad_request(err.to_string()),
    }
}

pub async fn create_branch(
    state : web::Data<AppFsState>,
    path : web::Path<(String, String, String)>,
    session : Session,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("Not Login".to_string()),
    };
    let repo = match state
        .info_to_uid(path.0.to_string(), path.1.to_string())
        .await
    {
        Ok(repo) => repo,
        Err(err) => return AppWrite::bad_request(err.to_string()),
    };
    match state.access(users.uid, repo.clone()).await {
        Ok(x) => {
            if !x {
                return AppWrite::forbidden("No Permission".to_string());
            }
        }
        Err(err) => return AppWrite::bad_request(err.to_string()),
    }
    match state.create_branch(repo, path.2.to_string()).await {
        Ok(_) => AppWrite::<String>::ok("Success".to_string()),
        Err(err) => AppWrite::bad_request(err.to_string()),
    }
}

pub async fn list_branch(
    state : web::Data<AppFsState>,
    path : web::Path<(String, String)>,
    session : Session,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("Not Login".to_string()),
    };
    let repo = match state
        .info_to_uid(path.0.to_string(), path.1.to_string())
        .await
    {
        Ok(repo) => repo,
        Err(err) => return AppWrite::bad_request(err.to_string()),
    };
    match state.access(users.uid, repo.clone()).await {
        Ok(x) => {
            if !x {
                return AppWrite::forbidden("No Permission".to_string());
            }
        }
        Err(err) => return AppWrite::bad_request(err.to_string()),
    }
    match state.list_branch(repo).await {
        Ok(branches) => AppWrite::ok(branches),
        Err(err) => AppWrite::bad_request(err.to_string()),
    }
}
