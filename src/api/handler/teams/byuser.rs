use crate::api::service::Service;
use crate::metadata::model::teams::teams::Model;
use crate::utils::r::R;
use actix_web::{web, Responder};
use uuid::Uuid;

#[utoipa::path(
    get,
    tag = "team",
    path = "/api/v1/team/users/{uid}",
    responses(
        (status = 200, description = "Ok"),
        (status = 500, description = "Other Error"),
    ),
)]
pub async fn api_team_by_user(
    service: web::Data<Service>,
    dto: web::Path<Uuid>
)
-> impl Responder
{
    match service.team.byuser(dto.into_inner()).await{
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