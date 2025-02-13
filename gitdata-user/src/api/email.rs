use actix_session::Session;
use actix_web::Responder;
use actix_web::web;
use lib_entity::session::USER_SESSION_KEY;
use lib_entity::session::UsersSessionModel;
use lib_entity::write::AppWrite;

use crate::server::AppUserState;
use crate::server::email::EmailAddParma;
use crate::server::email::MainEmailUpdate;

pub async fn main_email_can(session : Session, state : web::Data<AppUserState>) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.user_visible_email_can(users.uid).await {
        Ok(_) => AppWrite::ok(()),
        Err(_) => AppWrite::unauthorized("用户未登录".to_string()),
    }
}

pub async fn main_email_cannot(
    session : Session,
    state : web::Data<AppUserState>,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.user_visible_email_cannot(users.uid).await {
        Ok(_) => AppWrite::ok(()),
        Err(_) => AppWrite::unauthorized("用户未登录".to_string()),
    }
}

pub async fn main_email_is(session : Session, state : web::Data<AppUserState>) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.user_visible_email_is(users.uid).await {
        Ok(is) => AppWrite::ok(is),
        Err(_) => AppWrite::unauthorized("用户未登录".to_string()),
    }
}

pub async fn main_email_update(
    session : Session,
    state : web::Data<AppUserState>,
    parma : web::Json<MainEmailUpdate>,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.user_email_update(users.uid, parma.into_inner()).await {
        Ok(_) => AppWrite::ok(()),
        Err(_) => AppWrite::unauthorized("用户未登录".to_string()),
    }
}
pub async fn main_email_add(
    session : Session,
    state : web::Data<AppUserState>,
    parma : web::Json<EmailAddParma>,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.user_email_add(users.uid, parma.into_inner()).await {
        Ok(_) => AppWrite::ok(()),
        Err(_) => AppWrite::unauthorized("用户未登录".to_string()),
    }
}

pub async fn main_email_delete(
    session : Session,
    state : web::Data<AppUserState>,
    parma : web::Json<EmailAddParma>,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state
        .user_email_delete(users.uid, parma.into_inner().email)
        .await
    {
        Ok(_) => AppWrite::ok(()),
        Err(_) => AppWrite::unauthorized("用户未登录".to_string()),
    }
}

pub async fn main_email_list(session : Session, state : web::Data<AppUserState>) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.user_email_list(users.uid).await {
        Ok(list) => AppWrite::ok(list),
        Err(_) => AppWrite::unauthorized("用户未登录".to_string()),
    }
}
pub async fn main_email_set_primary(
    session : Session,
    state : web::Data<AppUserState>,
    parma : web::Json<EmailAddParma>,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state
        .user_email_set_primary(users.uid, parma.into_inner().email)
        .await
    {
        Ok(_) => AppWrite::ok(()),
        Err(_) => AppWrite::unauthorized("用户未登录".to_string()),
    }
}
pub async fn main_email_set_no_primary(
    session : Session,
    state : web::Data<AppUserState>,
    parma : web::Json<EmailAddParma>,
) -> impl Responder {
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state
        .user_email_set_no_primary(users.uid, parma.into_inner().email)
        .await
    {
        Ok(_) => AppWrite::ok(()),
        Err(_) => AppWrite::unauthorized("用户未登录".to_string()),
    }
}
