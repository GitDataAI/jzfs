use crate::api::app_writer::AppWrite;
use crate::api::middleware::session::SessionModel;
use crate::models::users::follower::Model;
use crate::server::MetaData;
use actix_session::Session;
use actix_web::{Responder, web};

pub async fn users_follower_add(
    session: Session,
    path: web::Path<String>,
    meta: web::Data<MetaData>,
) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<Option<String>>::fail(err.to_string()),
    };
    let target = match meta.users_info_username(path.into_inner()).await {
        Ok(target) => target,
        Err(err) => return AppWrite::<Option<String>>::fail(err.to_string()),
    };
    match meta.users_follower_add(session.uid, target.uid).await {
        Ok(_unused) => AppWrite::ok(None),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
pub async fn users_follower_del(
    session: Session,
    path: web::Path<String>,
    meta: web::Data<MetaData>,
) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<Option<String>>::fail(err.to_string()),
    };
    let target = match meta.users_info_username(path.into_inner()).await {
        Ok(target) => target,
        Err(err) => return AppWrite::<Option<String>>::fail(err.to_string()),
    };
    match meta.users_follower_del(session.uid, target.uid).await {
        Ok(_unused) => AppWrite::ok(None),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

pub async fn users_follower_get(session: Session, meta: web::Data<MetaData>) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<Vec<Model>>::fail(err.to_string()),
    };
    match meta.users_follower_list(session.uid).await {
        Ok(follower) => AppWrite::ok(follower),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

pub async fn users_follower_count(session: Session, meta: web::Data<MetaData>) -> impl Responder {
    let session = match SessionModel::authenticate(session).await {
        Ok(session) => session,
        Err(err) => return AppWrite::<usize>::fail(err.to_string()),
    };
    match meta.users_follower_list(session.uid).await {
        Ok(count) => AppWrite::ok(count.len()),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
