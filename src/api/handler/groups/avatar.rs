use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::dto::user_dto::UserAvatar;
use crate::api::handler::check_session;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "groups",
    path = "/api/v1/groups/avatar",
    responses(
        (status = 200, description = "Success", body = Vec<u8>),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_groups_avatar(
    session: Session,
    service: web::Data<MetaService>
)
    -> impl Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<String>::unauthorized(model.err().unwrap().to_string());
    }
    match service.user_service().avatar(model.unwrap().uid).await {
        Ok(result) => {
            AppWrite::ok(result)
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    post,
    tag = "groups",
    path = "/api/v1/groups/avatar",
    request_body = UserAvatar,
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_groups_avatar_upload(
    session: Session,
    service: web::Data<MetaService>,
    dto: web::Json<UserAvatar>
)
    -> impl Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<String>::unauthorized(model.err().unwrap().to_string());
    }
    match service.user_service().upload_avatar(model.unwrap().uid,dto.byte.clone()).await{
        Ok(_)=>{
            return AppWrite::ok_msg("ok".to_string());
        },
        Err(e)=>{
            return AppWrite::fail(e.to_string());
        }
    }
}