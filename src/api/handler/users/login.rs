use crate::api::app_write::AppWrite;
use crate::api::dto::user_dto::{UsersInner, UsersLoginEmail, UsersLoginUsername};
use crate::api::middleware::session::model::SessionModelKey;
use crate::metadata::service::MetaService;
use actix_session::Session;
use actix_web::{web, Responder};
use base64::Engine;

#[utoipa::path(
    post,
    tag = "users",
    path = "/api/v1/users/login/username",
    request_body = UsersLoginUsername,
    responses(
        ( status = 200, description = "Success" ),
        ( status = 400, description = "Bad Request" )
    )
)]
pub async fn api_users_login_name(
    session: Session,
    dto: web::Json<UsersInner>,
    service: web::Data<MetaService>)
    -> impl Responder
{
    let dto = match base64::prelude::BASE64_STANDARD.decode(dto.inner.as_bytes()){
        Ok(dto) => dto,
        Err(e) => {
            return AppWrite::<String>::fail(e.to_string());
        }
    };
    let dto: UsersLoginUsername = match serde_json::from_slice(&dto){
        Ok(dto) => dto,
        Err(e) => {
            return AppWrite::fail(e.to_string());
        }
    };
    match service.user_service().login_by_username(dto).await{
        Ok(info) => {
            session.insert(SessionModelKey, info).unwrap();
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
    path = "/api/v1/users/login/email",
    request_body = UsersLoginEmail,
    responses(
        ( status = 200, description = "Success" ),
        ( status = 400, description = "Bad Request" )
    )
)]
pub async fn api_users_login_email(
    session: Session,
    dto: web::Json<UsersInner>,
    service: web::Data<MetaService>)
    -> impl Responder
{
    let dto = match base64::prelude::BASE64_STANDARD.decode(dto.inner.as_bytes()){
        Ok(dto) => dto,
        Err(e) => {
            return AppWrite::fail(e.to_string());

        }
    };
    let dto: UsersLoginEmail = match serde_json::from_slice(&dto){
        Ok(dto) => dto,
        Err(e) => {
            return AppWrite::<String>::fail(e.to_string());

        }
    };
    match service.user_service().login_by_email(dto).await{
        Ok(info) => {
            session.insert(SessionModelKey, info).unwrap();
            AppWrite::ok_msg("ok".to_string())
        },
        Err(e) => {
            AppWrite::<String>::fail(e.to_string())
        }
    }
}