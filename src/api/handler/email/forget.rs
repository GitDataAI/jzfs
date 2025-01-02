use actix_web::{web, Responder};
use crate::api::dto::email::EmailCaptcha;
use crate::api::service::Service;
use crate::utils::r::R;


#[utoipa::path(
    post,
    tag = "email",
    path = "/api/v1/email/forget",
    request_body = EmailCaptcha,
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "User Not Exist"),
        (status = 405, description = "Other Error"),
    ),
)]
pub async fn api_email_forget(
    email: web::Json<EmailCaptcha>,
    service: web::Data<Service>,
)
-> impl Responder
{
    let check = service.users.check_email(email.email.clone()).await;
    if check.is_err(){
        return R::<String>{
            code: 400,
            msg: Option::from("[Error] User Not Exist".to_string()),
            data: None,
        }
    }
    if !check.unwrap() {
        return R::<String>{
            code: 400,
            msg: Option::from("[Error] User Not Exist".to_string()),
            data: None,
        }
    }
    match service.email.send_forget_token(email.email.clone()).await{
        Ok(_) => {
            R::<String>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: None,
            }
        },
        Err(e) => {
            R::<String>{
                code: 405,
                msg: Option::from(e.to_string()),
                data: None,
            }
       }
    }
}