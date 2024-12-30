use crate::api::app_writer::AppWrite;
use crate::api::handlers::users::options::EmailBind;
use crate::api::middleware::session::SessionModel;
use crate::models::users::email::Model;
use crate::server::MetaData;
use actix_session::Session;
use actix_web::{web, Responder};

pub async fn users_email_add(
    session: Session,
    meta: web::Data<MetaData>,
    inner: web::Json<EmailBind>,
) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<Option<String>>::fail(err.to_string()),
    };
    match meta.users_email_add(session.uid, inner.email.clone()).await {
        Ok(_unused) => AppWrite::ok(None),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
pub async fn users_email_del(
    session: Session,
    meta: web::Data<MetaData>,
    inner: web::Json<EmailBind>,
) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<Option<String>>::unauthorized(err.to_string()),
    };
    match meta.users_email_del(session.uid, inner.email.clone()).await {
        Ok(_unused) => AppWrite::ok(None),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

pub async fn users_email_get(session: Session, meta: web::Data<MetaData>) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<Vec<Model>>::unauthorized(err.to_string()),
    };
    match meta.users_email_list(session.uid).await {
        Ok(emails) => AppWrite::ok(emails),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
