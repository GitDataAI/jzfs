use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "users",
    path = "/api/v1/users/info/{user}",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_users_info(
    service: web::Data<MetaService>,
    user: web::Path<String>
) -> impl Responder
{
    
    let user_id = match service.user_service().username_to_uid(user.into_inner()).await{
        Ok(x) => x,
        Err(e) => return AppWrite::fail(e.to_string())
    };
    let model = service.user_service().info(user_id).await;
    match model{
        Ok(model) => AppWrite::ok(model),
        Err(e) => AppWrite::fail(e.to_string())
    }
}