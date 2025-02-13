use actix_session::Session;
use actix_web::Responder;
use lib_entity::session::USER_SESSION_KEY;
use lib_entity::session::UsersSessionModel;
use lib_entity::write::AppWrite;

use crate::server::AppUserState;
use crate::server::ssh_keys::UserCreateSshKey;
use crate::server::ssh_keys::UserTokenDelete;
use crate::web;

pub async fn user_add_ssh_key(
    session : Session,
    state : web::Data<AppUserState>,
    parma : web::Json<UserCreateSshKey>,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(user)) => user,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.create_ssh_key(users.uid, parma.into_inner()).await {
        Ok(_) => AppWrite::ok(()),
        Err(err) => AppWrite::error(err.to_string()),
    }
}

pub async fn user_list_ssh_key(
    session : Session,
    state : web::Data<AppUserState>,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(user)) => user,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.list_ssh_key(users.uid).await {
        Ok(keys) => AppWrite::ok(keys),
        Err(err) => AppWrite::error(err.to_string()),
    }
}

pub async fn user_delete_ssh_key(
    session : Session,
    state : web::Data<AppUserState>,
    parma : web::Json<UserTokenDelete>,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(user)) => user,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.delete_ssh_key(users.uid, parma.into_inner()).await {
        Ok(_) => AppWrite::ok(()),
        Err(err) => AppWrite::error(err.to_string()),
    }
}
