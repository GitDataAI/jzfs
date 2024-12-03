use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::dto::group::GroupCreate;
use crate::api::middleware::session::{SessionModel, SESSION_USER_KEY};
use crate::api::service::Service;
use crate::utils::r::R;

pub async fn api_group_create(
    dto: web::Json<GroupCreate>,
    service: web::Data<Service>,
    session: Session
)
    -> impl Responder
{
    let model = session.get::<SessionModel>(SESSION_USER_KEY).unwrap().unwrap();
    match service.group.create(dto.into_inner(), model.uid).await{
        Ok(_) => {
            R::<String>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: None
            }
        }
        Err(e) => {
            R::<String>{
                code: 500,
                msg: Option::from(e.to_string()),
                data: None
            }
        }
    }
}

