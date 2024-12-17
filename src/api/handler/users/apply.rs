use actix_session::Session;
use actix_web::{web, Responder};
use base64::Engine;
use crate::api::app_write::AppWrite;
use crate::api::dto::user_dto::{UserApply, UsersInner};
use crate::api::middleware::access::ALLOW_NEXT_KEY;
use crate::metadata::service::MetaService;


#[utoipa::path(
    post,
    tag = "users",
    path = "/api/v1/users/apply",
    request_body = UserApply,
    responses(
        ( status = 200, description = "Success" ),
        ( status = 400, description = "Bad Request" )
    )
)]
pub async fn api_users_apply(
    session: Session,
    dto: web::Json<UsersInner>,
    service: web::Data<MetaService>
) -> impl Responder
{
    let allow = session.get::<bool>(ALLOW_NEXT_KEY).unwrap();
    if allow.is_none(){
        return AppWrite::<String>::fail("[Error] Not Allow Next".to_string());
    }
    let dto = match base64::prelude::BASE64_STANDARD.decode(dto.inner.as_bytes()){
        Ok(dto) => dto,
        Err(e) => {
           return AppWrite::fail(e.to_string());
        }
    };
    let dto: UserApply = match serde_json::from_slice(&dto){
        Ok(dto) => dto,
        Err(e) => {
           return AppWrite::fail(e.to_string());
        }
    };
    match service.user_service().apply(dto).await{
        Ok(_info) => {
            session.remove(ALLOW_NEXT_KEY);
            AppWrite::ok_msg("ok".to_string())
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}