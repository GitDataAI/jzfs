use actix_session::Session;
use actix_web::{web, Responder};
use base64::Engine;
use crate::api::dto::users::{UserApply, UsersInner};
use crate::api::middleware::session::ALLOW_NEXT_KEY;
use crate::api::service::Service;
use crate::utils::r::R;

#[utoipa::path(
    post,
    tag = "users",
    path = "/api/v1/users/apply",
    request_body = UsersInner,
    responses(
            (status = 200, description = "Apply Success"),
            (status = NOT_FOUND, description = "Pet was not found")
    ),
)]
pub async fn api_user_apply(
    session: Session,
    dto: web::Json<UsersInner>,
    service: web::Data<Service>
) -> impl Responder
{
    let allow = session.get::<bool>(ALLOW_NEXT_KEY).unwrap();
    if allow.is_none(){
        return R::<String>{
            code: 400,
            msg: Option::from("[Error] Not Allow Next".to_string()),
            data: None,
        }
    }
    let dto = match base64::prelude::BASE64_STANDARD.decode(dto.inner.as_bytes()){
        Ok(dto) => dto,
        Err(e) => {
            return R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    };
    let dto: UserApply = match serde_json::from_slice(&dto){
        Ok(dto) => dto,
        Err(e) => {
            return R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    };
    match service.users.apply(dto).await{
        Ok(_info) => {
            session.remove(ALLOW_NEXT_KEY);
            R::<String>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: None,
            }
        },
        Err(e) => {
            R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}