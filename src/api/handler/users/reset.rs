use actix_session::Session;
use actix_web::{web, Responder};
use base64::Engine;
use crate::api::app_write::AppWrite;
use crate::api::dto::user_dto::{UserResetPasswd, UserResetPassword, UsersInner};
use crate::api::middleware::session::model::{SessionModel, SessionModelKey};
use crate::metadata::service::MetaService;

#[utoipa::path(
    post,
    tag = "users",
    path = "/api/v1/users/reset/profile",
    request_body = UserResetPasswd,
    responses(
        ( status = 200, description = "Success" ),
        ( status = 400, description = "Bad Request" )
    )
)]
pub async fn api_user_reset_passwd_profile(
    session: Session,
    dto: web::Json<UsersInner>,
    service: web::Data<MetaService>
) -> impl Responder
{
    let model = session.get::<SessionModel>(SessionModelKey).unwrap();
    if model.is_none(){
        return AppWrite::<String>::unauthorized("[Error] Not Login".to_string());
    }
    let model = model.unwrap();
    let dto = match base64::prelude::BASE64_STANDARD.decode(dto.inner.as_bytes()){
        Ok(dto) => dto,
        Err(e) => {
            return AppWrite::fail(e.to_string());
        }
    };
    let dto: UserResetPasswd = match serde_json::from_slice(&dto){
        Ok(dto) => dto,
        Err(e) => {
            return AppWrite::fail(e.to_string());
        }
    };
    match service.user_service().reset(dto,model.uid).await{
        Ok(_) => {
            AppWrite::ok_msg("ok".to_string())
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    post,
    tag = "users",
    path = "/api/v1/users/reset/forget",
    request_body = UserResetPassword,
    responses(
        ( status = 200, description = "Success" ),
        ( status = 400, description = "Bad Request" )
    )
)]

pub async fn api_user_reset_passwd_forget(
    dto: web::Json<UsersInner>,
    service: web::Data<MetaService>,
)
    -> impl Responder
{
    let dto = match base64::prelude::BASE64_STANDARD.decode(dto.inner.as_bytes()){
        Ok(dto) => dto,
        Err(e) => {
            return AppWrite::<String>::fail(e.to_string())
        }
    };
    let dto: UserResetPassword = match serde_json::from_slice(&dto){
        Ok(dto) => dto,
        Err(e) => {
            return AppWrite::fail(e.to_string())
        }
    };
    match service.user_service().reset_by_token(dto).await{
        Ok(_) => {
            AppWrite::ok_msg("ok".to_string())
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}
