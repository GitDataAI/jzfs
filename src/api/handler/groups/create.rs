use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::dto::groups_dto;
use crate::metadata::service::MetaService;


#[utoipa::path(
    post,
    tag = "groups",
    path = "/api/v1/groups",
    request_body = groups_dto::GroupCreate,
    responses(
        (status = 200, description = "Success", body = String),
        (status = 401, description = "Not Login"),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn api_groups_create(
    session: Session,
    dto: web::Json<groups_dto::GroupCreate>,
    service: web::Data<MetaService>,
)
-> impl Responder
{
    let session_model = crate::api::handler::check_session(session).await;
    if session_model.is_err(){
        return AppWrite::<String>::unauthorized(session_model.err().unwrap().to_string())
    }
    match service.group_service().create(dto.into_inner(), session_model.unwrap().uid).await{
        Ok(_) => AppWrite::<String>::success("success".to_string()),
        Err(e) => AppWrite::<String>::error(e.to_string())
    }
}