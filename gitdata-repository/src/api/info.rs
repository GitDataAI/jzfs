use actix_web::Responder;
use actix_web::web;
use lib_entity::write::AppWrite;

use crate::service::AppFsState;

pub async fn repository_info(
    parma : web::Path<(String, String)>,
    service : web::Data<AppFsState>,
) -> impl Responder {
    match service.info(parma.0.clone(), parma.1.clone()).await {
        Ok(repo) => AppWrite::ok(repo),
        Err(err) => AppWrite::error(err.to_string()),
    }
}
