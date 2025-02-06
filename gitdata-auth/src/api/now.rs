use actix_web::{web, Responder};
use actix_session::Session;
use lib_entity::session::{UsersSessionModel, USER_SESSION_KEY};
use lib_entity::write::AppWrite;
use crate::server::AppAuthState;

pub async fn auth_now_session(
    session: Session,
)
    -> impl Responder
{
    match session.get::<UsersSessionModel>(USER_SESSION_KEY) {  
        Ok(Some(user)) => {
            AppWrite::ok(user)
        },
        _ => {
            AppWrite::unauthorized("未登录".to_string())
        }
    }
}

pub async fn auth_now_logout(
    session: Session,
)
    -> impl Responder
{
    session.purge();
    AppWrite::<()>::ok_msg("退出成功".to_string())
}

pub async fn auth_now_users(
    session: Session,
    state: web::Data<AppAuthState>,
)
    -> impl Responder
{
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {  
        Ok(Some(user)) => {
            user
        },
        _ => {
            return AppWrite::unauthorized("未登录".to_string());
        }
    };
    match state.now_user(users.uid).await {
        Ok(user) => {
            AppWrite::ok(user)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}
