use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "users",
    path = "/api/v1/users/{username}/starred",
    responses(
        ( status = 200, description = "Success" ),
        ( status = 400, description = "Bad Request" )
    )
)]
pub async fn api_users_starred(
    service: web::Data<MetaService>,
    path: web::Path<String>
)
 -> impl Responder
{
    let username = path.into_inner();
    let uid = match service.user_service().username_to_uid(username).await{
        Ok(uid) => uid,
        Err(e) => return AppWrite::error(e.to_string())
    };
    match service.user_service().star(uid).await{
        Ok(data) => {
            AppWrite::ok(data)
        }
        Err(e) => {
            AppWrite::error(e.to_string())
        }
    }
}