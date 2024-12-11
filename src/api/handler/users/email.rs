use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::dto::email::EmailBind;
use crate::api::service::Service;
use crate::utils::r::R;

#[utoipa::path(
    post,
    tag = "user",
    path = "/api/v1/user/email/bind",
    request_body = EmailBind,
    responses(
            (status = 200, description = "OK"),
            (status = 401, description = "Unauthorized"),
            (status = 405, description = "Other Error"),
    ),
)]
pub async fn api_user_email_bind(
    session: Session,
    service: web::Data<Service>,
    email: web::Json<EmailBind>,
) 
    -> impl Responder 
{
    let session = service.check.check_session(session).await;
    if !session.is_ok() {
        return R::<String> {
            code: 401,
            data: None,
            msg: Option::from("unauthorized".to_string()),
        };
    }
    let session = session.unwrap();
    match service.email.bind(email.email.clone(), session.uid, email.name.clone()).await {
        Ok(_) => {
            return R {
                code: 200,
                data: None,
                msg: Option::from("ok".to_string()),
            };
        },
        Err(e) => {
            return R {
                code: 405,
                data: None,
                msg: Option::from(e.to_string()),
            };
        }
   }
}

#[utoipa::path(
    delete,
    tag = "user",
    path = "/api/v1/user/email/unbind",
    request_body = EmailBind,
    responses(
            (status = 200, description = "OK"),
            (status = 401, description = "Unauthorized"),
            (status = 405, description = "Other Error"),
    ),
)]
pub async fn api_user_email_unbind(
    session: Session,
    service: web::Data<Service>,
    email: web::Json<EmailBind>,
) 
    -> impl Responder 
{
    let session = service.check.check_session(session).await;
    if !session.is_ok() {
        return R::<String> {
            code: 401,
            data: None,
            msg: Option::from("unauthorized".to_string()),
        };
    }
    let session = session.unwrap();
    match service.email.unbind(email.email.clone(), session.uid).await {
        Ok(_) => {
            return R {
                code: 200,
                data: None,
                msg: Option::from("ok".to_string()),
            };
        },
        Err(e) => {
            return R {
                code: 405,
                data: None,
                msg: Option::from(e.to_string()),
            }
        }
    }
}