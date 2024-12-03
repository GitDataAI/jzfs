use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::ov::users::UserOv;
use crate::api::service::Service;
use crate::utils::r::R;

pub async fn api_owner_info(
    session: Session,
    service: web::Data<Service>
)
-> impl Responder
{
    let model = service.check.check_session(session).await;
    if model.is_err(){
        return R::<UserOv>{
            code: 400,
            msg: Option::from("[Error] User Not Exist".to_string()),
            data: None,
        }
    }
    match service.users.info(model.unwrap().uid).await{
        Ok(result)=>{
            R{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Some(result),
            }
        },
        Err(e)=>{
            R::<UserOv>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}