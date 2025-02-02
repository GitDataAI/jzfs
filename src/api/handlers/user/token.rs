use crate::api::app_writer::AppWrite;
use crate::api::middleware::session::SessionModel;
use crate::models::users::token_key::Model;
use crate::server::MetaData;
use actix_session::Session;
use actix_web::{Responder, web};

pub async fn users_token_get(session: Session, meta: web::Data<MetaData>) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<Vec<Model>>::fail(err.to_string()),
    };
    match meta.users_token_list(session.uid).await {
        Ok(tokens) => AppWrite::ok(tokens),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
