use actix_session::Session;
use actix_web::{web, Responder};
use uuid::Uuid;
use crate::api::dto::team::TeamCreate;
use crate::api::service::Service;
use crate::utils::r::R;


#[utoipa::path(
    post,
    tag = "team",
    path = "/api/v1/team/{group}/create",
    request_body = TeamCreate,
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "Group Not Exist"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Other Error"),
    ),
)]
pub async fn api_teams_create(
    session: Session,
    group: web::Path<Uuid>,
    dto: web::Json<TeamCreate>,
    service: web::Data<Service>
)
-> impl Responder
{   
    let model = service.check.check_session(session).await;
    if model.is_err(){
        return R::<String>{
            code: 401,
            msg: Option::from("unauthorized".to_string()),
            data: None
        }
    }
    if service.check.check_exits_group(group.clone()).await{
        return R{
            code: 400,
            msg: Option::from("group not exits".to_string()),
            data: None
        }
    }
    match service.team.create_team(dto.into_inner(), model.unwrap().uid, group.into_inner()).await{
        Ok(_) => R{
            code: 200,
            msg: Option::from("success".to_string()),
            data: None
        },
        Err(e) => R{
            code: 500,
            msg: Option::from(e.to_string()),
            data: None
        }
    }
}