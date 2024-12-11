use actix_session::Session;
use actix_web::web;
use uuid::Uuid;
use crate::api::service::Service;

#[utoipa::path(
    get,
    tag = "owner",
    path = "/api/v1/owner/watcher",
    responses(
            (status = 200, description = "Ok", body = Vec<Uuid>),
            (status = 401, description = "Not Login"),
            (status = 402, description = "User Not Exist"),
            (status = 405, description = "Other Error"),
    ),
)]
pub async fn api_owner_watcher(
    session: Session,
    service: web::Data<Service>
)
-> impl actix_web::Responder
{
    let model = service.check.check_session(session).await;
    if model.is_err(){
        return crate::utils::r::R::<Vec<Uuid>>{
            code: 402,
            msg: Option::from("[Error] User Not Exist".to_string()),
            data: None,
        }
    }
    match service.users.wacther(model.unwrap().uid).await{
        Ok(result)=>{
            crate::utils::r::R::<Vec<Uuid>>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Some(result),
            }
        },
        Err(e)=>{
            crate::utils::r::R{
                code: 405,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}