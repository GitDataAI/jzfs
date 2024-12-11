use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::ov::users::UserOv;
use crate::api::service::Service;
use crate::utils::r::R;

#[utoipa::path(
    get,
    tag = "owner",
    path = "/api/v1/owner/setting",
    responses(
            (status = 200, description = "OK", body = UserOv),
            (status = 401, description = "Not Login"),
            (status = 500, description = "Internal Server Error"),
    ),
)]
pub async fn api_owner_setting(
    session: Session,
    service: web::Data<Service>
)
-> impl Responder
{
    let model = service.check.check_session(session).await;
    if model.is_err(){
        return R::<UserOv>{
            code: 401,
            msg: Option::from("Unauthorized".to_string()),
            data: None,
        }
    }
    let model = model.unwrap();
    let model = service.users.info(model.uid).await;
    if model.is_err(){
        return R{
            code: 500,
            msg: Option::from("Internal Server Error".to_string()),
            data: None,
        }
    }
    R{
        code: 200,
        msg: Option::from("OK".to_string()),
        data: Some(model.unwrap()),
    }
}