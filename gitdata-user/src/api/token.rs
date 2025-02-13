use actix_session::Session;
use actix_web::Responder;
use actix_web::web;
use lib_entity::session::USER_SESSION_KEY;
use lib_entity::session::UsersSessionModel;
use lib_entity::write::AppWrite;

use crate::server::AppUserState;
use crate::server::tokens::UserCreateToken;
use crate::server::tokens::UserTokenDelete;

pub async fn create_token(
    session : Session,
    state : web::Data<AppUserState>,
    parma : web::Json<UserCreateToken>,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(user)) => user,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.create_token(users.uid, parma.into_inner()).await {
        Ok(token) => AppWrite::ok(token),
        Err(err) => AppWrite::error(err.to_string()),
    }
}

pub async fn list_token(session : Session, state : web::Data<AppUserState>) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(user)) => user,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.token_list(users.uid).await {
        Ok(token) => AppWrite::ok(token),
        Err(err) => AppWrite::error(err.to_string()),
    }
}

pub async fn delete_token(
    session : Session,
    state : web::Data<AppUserState>,
    parma : web::Json<UserTokenDelete>,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(user)) => user,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.delete_token(users.uid, parma.into_inner()).await {
        Ok(_) => AppWrite::ok("删除成功".to_string()),
        Err(err) => AppWrite::error(err.to_string()),
    }
}
