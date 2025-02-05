use actix_web::{web, Responder};
use actix_session::Session;
use lib_entity::session::{UsersSessionModel, USER_SESSION_KEY};
use lib_entity::write::AppWrite;
use crate::server::AppUserState;
use crate::server::follow::FollowParma;

pub async fn users_follow(
    session: Session,
    state: web::Data<AppUserState>,
    parma: web::Json<FollowParma>,
) -> impl Responder
{
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.follow(users.uid, parma.uid).await {
        Ok(_) => AppWrite::ok("关注成功".to_string()),
        Err(err) => AppWrite::error(err.to_string()),
    }
}

pub async fn users_unfollow(
    session: Session,
    state: web::Data<AppUserState>,
    parma: web::Json<FollowParma>,
) -> impl Responder
{
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.unfollow(users.uid, parma.uid).await {
        Ok(_) => AppWrite::ok("取消关注成功".to_string()),
        Err(err) => AppWrite::error(err.to_string()),
    }
}

pub async fn users_follow_list(
    session: Session,
    state: web::Data<AppUserState>,
) -> impl Responder
{
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.follow_list(users.uid).await {
        Ok(list) => AppWrite::ok(list),
        Err(err) => AppWrite::error(err.to_string()),
    }
}
