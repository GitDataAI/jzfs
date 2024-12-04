use crate::api::service::Service;
use crate::metadata::model::teams::teams::Model;
use crate::utils::r::R;
use actix_web::{web, Responder};
use uuid::Uuid;

#[utoipa::path(
    get,
    tag = "team",
    path = "/api/v1/team/{uid}/info",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "Team Not Exist"),
        (status = 500, description = "Other Error"),
    ),
)]
pub async fn api_team_info(
    service: web::Data<Service>,
    uid: web::Path<Uuid>
)
-> impl Responder
{
    match service.team.info(uid.into_inner()).await{
        Ok(data) => {
            R::<Model>{
                code: 200,
                msg: Option::from("success".to_string()),
                data: Some(data),
            }
        },
        Err(e) => {
            R{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
   }
}
