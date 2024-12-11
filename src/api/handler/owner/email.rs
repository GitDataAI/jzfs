use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::service::Service;
use crate::metadata::model::users::users_email::Model;
use crate::utils::r::R;

#[utoipa::path(
    get,
    tag = "owner",
    path = "/api/v1/owner/email",
    responses(
            (status = 200, description = "OK", body = Vec<Model>),
            (status = 401, description = "Not Login"),
            (status = 402, description = "User Not Exist"),
            (status = 403, description = "Get Teams Failed"),
            (status = 405, description = "Other Error"),
    ),
)]
pub async fn api_owner_email(
    session: Session,
    service: web::Data<Service>
)
->impl Responder
{
    let model = service.check.check_session(session).await;
    if model.is_err(){
        return R::<Vec<Model>>{
            code: 402,
            msg: Option::from("[Error] User Not Exist".to_string()),
            data: None,
        }
    }
    match service.users.email(model.unwrap().uid).await{
        Ok(result)=>{
            R::<Vec<Model>>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Some(result),
            }
        },
        Err(e)=>{
            R{
                code: 405,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}