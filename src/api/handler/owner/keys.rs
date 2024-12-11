use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::dto::users::UserKeyList;
use crate::api::service::Service;
use crate::utils::r::R;


#[utoipa::path(
    get,
    tag = "owner",
    path = "/api/v1/owner/keys",
    responses(
            (status = 200, description = "OK", body = Vec<UserKeyList>),
            (status = 401, description = "Not Login"),
            (status = 402, description = "User Not Exist"),
            (status = 403, description = "Get Keys Failed"),
            (status = 405, description = "Other Error"),
    ),
)]
pub async fn api_owner_keys(
    session: Session,
    service: web::Data<Service>
)
-> impl Responder
{
    let model = service.check.check_session(session).await;
    if model.is_err(){
        return R::<Vec<UserKeyList>>{
            code: 402,
            msg: Option::from("[Error] User Not Exist".to_string()),
            data: None,
        }
    }
    match service.users.list_key(model.unwrap().uid).await{
        Ok(result)=>{
            R{
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