use std::string::ToString;
use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::service::Service;
use crate::metadata::model::teams::teams;
use crate::utils::r::R;

#[utoipa::path(
    get,
    tag = "owner",
    path = "/api/v1/owner/team",
    responses(
            (status = 200, description = "Ok"),
            (status = 401, description = "Not Login"),
            (status = 402, description = "User Not Exist"),
            (status = 405, description = "Other Error"),
    ),
)]
pub async fn api_owner_teams(
    session: Session,
    service: web::Data<Service>
)
-> impl Responder
{
    let model = service.check.check_session(session).await;
    if model.is_err() {
        return R::<Vec<teams::Model>> {
            code: 402,
            msg: Option::from("[Error] User Not Exist".to_string()),
            data: None,
        }
    }
    match service.team.byuser(model.unwrap().uid).await {
        Ok(x) => {
            return R::<Vec<teams::Model>> {
                code: 200,
                msg: Option::from("[Success] Get Teams Success".to_string()),
                data: Some(x),
            }
        },
        Err(e) => {
            R{
                code: 405,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}