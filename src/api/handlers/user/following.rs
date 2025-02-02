use crate::api::app_writer::AppWrite;
use crate::api::middleware::session::SessionModel;
use crate::models::users::follower::Model;
use crate::server::MetaData;
use actix_session::Session;
use actix_web::{Responder, web};

pub async fn users_following_get(session: Session, meta: web::Data<MetaData>) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<Vec<Model>>::fail(err.to_string()),
    };
    match meta.users_following_list(session.uid).await {
        Ok(following) => AppWrite::ok(following),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

pub async fn users_following_count(session: Session, meta: web::Data<MetaData>) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<usize>::fail(err.to_string()),
    };
    match meta.users_following_list(session.uid).await {
        Ok(count) => AppWrite::ok(count.len()),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
