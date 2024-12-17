use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::dto::user_dto::{UserOv, UserUpdate};
use crate::api::handler::check_session;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "user",
    path = "/api/v1/user/setting",
    responses(
        (status = 200, description = "Success", body = UserOv),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_setting_get(
    session: Session,
    service: web::Data<MetaService>
)
    -> impl Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<UserOv>::unauthorized(model.err().unwrap().to_string())
    }
    let model = model.unwrap();
    let model = service.user_service().info(model.uid).await;
    if model.is_err(){
        AppWrite::fail(model.err().unwrap().to_string())
    }else { 
        AppWrite::ok(model.unwrap())
    }
}
#[utoipa::path(
    patch,
    tag = "user",
    path = "/api/v1/user/setting",
    request_body = UserUpdate,
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_setting_patch(
    session: Session,
    service: web::Data<MetaService>,
    dto: web::Json<UserUpdate>
)
    -> impl Responder
{
    let user = check_session(session).await;
    if user.is_err() {
        return AppWrite::<String>::unauthorized(user.err().unwrap().to_string());
    }
    let user = user.unwrap();
    match service.user_service().update_by_uid(user.uid, dto.into_inner()).await {
        Ok(_info) => {
            AppWrite::ok_msg("ok".to_string())
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}