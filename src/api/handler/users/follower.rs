use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::dto::user_dto::UserFollowerOv;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "users",
    path = "/api/v1/users/once/{user}/followers",   
    params(
        ( "user",Path, description = "User Name"),
    ),
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_users_followed(
    service: web::Data<MetaService>,
    user: web::Path<String>
) -> impl Responder
{
    let user_id = match service.user_service().username_to_uid(user.into_inner()).await{
        Ok(x) => x,
        Err(e) => return AppWrite::<Vec<UserFollowerOv>>::fail(e.to_string())
    };
    let models = service.user_service().followed(user_id).await;
    match models{
        Ok(models) => AppWrite::ok(models),
        Err(e) => AppWrite::fail(e.to_string())
    }
}

#[utoipa::path(
    get,
    tag = "users",
    path = "/api/v1/users/once/{user}/following",
    params(
        ( "user",Path, description = "User Name"),
    ),
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_users_following(
    service: web::Data<MetaService>,
    user: web::Path<String>
) -> impl Responder
{
    let user_id = match service.user_service().username_to_uid(user.into_inner()).await{
        Ok(x) => x,
        Err(e) => return AppWrite::<Vec<UserFollowerOv>>::fail(e.to_string())
    };
    let models = service.user_service().followers(user_id).await;
    match models{
        Ok(models) => AppWrite::ok(models),
        Err(e) => AppWrite::fail(e.to_string())
    }
}