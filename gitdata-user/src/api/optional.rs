use actix_web::{web, Responder};
use actix_session::Session;
use lib_entity::session::{UsersSessionModel, USER_SESSION_KEY};
use lib_entity::users::users::UpdateOption;
use lib_entity::write::AppWrite;
use crate::server::AppUserState;

pub async fn update_optional(
    session: Session,
    state: web::Data<AppUserState>,
    parma: web::Json<UpdateOption>,
) -> impl Responder
{
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.update_optional(users.uid, parma.into_inner()).await {
        Ok(_) => AppWrite::ok("更新成功".to_string()),
        Err(err) => AppWrite::error(err.to_string()),
    }
}
pub async fn acquire_optional(
    session: Session,
    state: web::Data<AppUserState>,
) -> impl Responder
{
    let users = match session.get::<UsersSessionModel>(USER_SESSION_KEY) {
        Ok(Some(users)) => users,
        _ => return AppWrite::unauthorized("用户未登录".to_string()),
    };
    match state.acquire_optional(users.uid).await {
        Ok(optional) => AppWrite::ok(optional),
        Err(err) => AppWrite::error(err.to_string()),
    }
}