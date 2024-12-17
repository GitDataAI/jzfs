use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::dto::email_dto::EmailBind;
use crate::api::handler::check_session;
use crate::metadata::model::users::UsersEmail;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "user",
    path = "/api/v1/user/email",
    responses(
        (status = 200, description = "Success", body = Vec<UsersEmail>),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_email(
    session: Session,
    service: web::Data<MetaService>
)
    ->impl Responder
{
    let model = check_session(session).await;
    if model.is_err(){
        return AppWrite::<Vec<UsersEmail>>::unauthorized(model.err().unwrap().to_string())
    }
    match service.user_service().email(model.unwrap().uid).await {
        Ok(result) => {
            AppWrite::ok(result)
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    post,
    tag = "user",
    path = "/api/v1/user/email",
    request_body = EmailBind,
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_email_bind(
    session: Session,
    service: web::Data<MetaService>,
    email: web::Json<EmailBind>,
)
    -> impl Responder
{
    let session = check_session(session).await;
    if !session.is_ok() {
        return AppWrite::<String>::unauthorized(session.err().unwrap().to_string());
    }
    let session = session.unwrap();
    match service.user_service().bind(email.email.clone(), session.uid, email.name.clone()).await {
        Ok(_) => {
            AppWrite::ok_msg("ok".to_string())
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    delete,
    tag = "user",
    path = "/api/v1/user/email",
    request_body = EmailBind,
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_email_unbind(
    session: Session,
    service: web::Data<MetaService>,
    email: web::Json<EmailBind>,
)
    -> impl Responder
{
    let session = check_session(session).await;
    if !session.is_ok() {
        return AppWrite::<String>::unauthorized(session.err().unwrap().to_string());
    }
    let session = session.unwrap();
    match service.user_service().unbind(email.email.clone(), session.uid).await {
        Ok(_) => {
            AppWrite::ok_msg("ok".to_string())
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}