use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::service::Service;
use crate::metadata::model::teams::teams;
use crate::utils::r::R;

pub async fn api_owner_teams(
    session: Session,
    service: web::Data<Service>
)
-> impl Responder
{
    let model = service.check.check_session(session).await;
    if model.is_err() {
        return R::<Vec<teams::Model>> {
            code: 400,
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
        Err(_) => {
            return R::<Vec<teams::Model>> {
                code: 400,
                msg: Option::from("[Error] Get Teams Failed".to_string()),
                data: None,
            }
        }
    }
}