use crate::api::app_writer::AppWrite;
use crate::api::handlers::repos::RepoCreateOwnerList;
use crate::api::middleware::session::SessionModel;
use crate::server::MetaData;
use actix_session::Session;
use actix_web::{Responder, web};

pub async fn repo_owner_check(session: Session, meta: web::Data<MetaData>) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(err) => return AppWrite::<Vec<RepoCreateOwnerList>>::unauthorized(err.to_string()),
    };
    match meta.repo_owner_list_check(model.uid).await {
        Ok(result) => AppWrite::ok(result),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

pub async fn check_repo_name(
    name: web::Path<(String, String)>,
    meta: web::Data<MetaData>,
) -> impl Responder {
    let (owner, name) = name.into_inner();
    match meta.repo_info(owner, name).await {
        Ok(_result) => AppWrite::<bool>::ok(false),
        Err(_err) => AppWrite::ok(true),
    }
}
