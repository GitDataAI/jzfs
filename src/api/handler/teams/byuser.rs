use actix_web::{web, Responder};
use crate::api::dto::team::TeamUid;
use crate::api::service::Service;
use crate::metadata::model::teams::teams::Model;
use crate::utils::r::R;

#[utoipa::path(
    get,
    tag = "team",
    path = "/api/v1/team/byuser",
    request_body = TeamUid,
    responses(
        (status = 200, description = "Ok"),
        (status = 500, description = "Other Error"),
    ),
)]
pub async fn api_team_by_user(
    service: web::Data<Service>,
    dto: web::Json<TeamUid>
)
-> impl Responder
{
    match service.team.byuser(dto.uid).await{
        Ok(data) => {
            R::<Vec<Model>>{
                code: 200,
                msg: Option::from("success".to_string()),
                data: Some(data),
            }
        },
        Err(e) => {
            R{
                code: 500,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}