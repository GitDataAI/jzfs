use actix_session::Session;
use actix_web::web;
use crate::api::app_write::AppWrite;
use crate::api::dto::user_dto::{UserFollow, UserFollowerOv};
use crate::api::handler::check_session;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "user",
    path = "/api/v1/user/follower",
    responses(
        (status = 200, description = "Success", body = Vec<UserFollowerOv>),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_follower(
    session: Session,
    service: web::Data<MetaService>
)
    -> impl actix_web::Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<Vec<UserFollowerOv>>::unauthorized(model.err().unwrap().to_string())
    }
    match service.user_service().followers(model.unwrap().uid).await{
        Ok(result)=>{
            AppWrite::ok(result)
        },
        Err(e)=>{
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    get,
    tag = "user",
    path = "/api/v1/user/followed",
    responses(
        (status = 200, description = "Success", body = Vec<UserFollowerOv>),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_followed(
    session: Session,
    service: web::Data<MetaService>
)
    -> impl actix_web::Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<Vec<UserFollowerOv>>::unauthorized(model.err().unwrap().to_string())
    }
    match service.user_service().followed(model.unwrap().uid).await{
        Ok(result)=>{
            AppWrite::ok(result)
        },
        Err(e)=>{
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    post,
    tag = "user",
    path = "/api/v1/user/follow",
    request_body = UserFollow,
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_follow(
    session: Session,
    service: web::Data<MetaService>,
    target: web::Path<String>
)
    -> impl actix_web::Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<String>::unauthorized(model.err().unwrap().to_string());
    }
    let target = match service.user_service().username_to_uid(target.into_inner()).await{
        Ok(result)=>result,
        Err(e)=>{
            return AppWrite::fail(e.to_string())
        }
    };
    match service.user_service().follow(model.unwrap().uid, target).await {
        Ok(_) => {
            AppWrite::ok_msg("ok".to_string())
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    delete,
    tag = "user",
    path = "/api/v1/user/unfollow",
    request_body = UserFollow,
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_unfollow(
    session: Session,
    service: web::Data<MetaService>,
    target: web::Path<String>
)
    -> impl actix_web::Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<String>::unauthorized(model.err().unwrap().to_string());
    }
    let target = match service.user_service().username_to_uid(target.into_inner()).await{
        Ok(result)=>result,
        Err(e)=>{
            return AppWrite::fail(e.to_string())
        }
    };
    match service.user_service().unfollow(model.unwrap().uid, target).await {
        Ok(_) => {
            AppWrite::ok_msg("ok".to_string())
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}