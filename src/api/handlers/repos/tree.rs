use crate::api::app_writer::AppWrite;
use crate::git::git::options::BlobTreeMsg;
use crate::server::MetaData;
use actix_web::web::{Data, Path, Query};
use actix_web::Responder;

pub async fn repo_tree(
    path: Path<(String, String, String)>,
    meta: Data<MetaData>,
) -> impl Responder {
    match meta
        .repo_tree(path.0.clone(), path.1.clone(), path.2.clone())
        .await
    {
        Ok(items) => AppWrite::<BlobTreeMsg>::ok(items),
        Err(e) => AppWrite::error(e.to_string()),
    }
}

pub async fn repo_tree_sha(
    path: Path<(String, String, String, String)>,
    meta: Data<MetaData>,
) -> impl Responder {
    match meta
        .repo_tree_sha(
            path.0.clone(),
            path.1.clone(),
            path.2.clone(),
            path.3.clone(),
        )
        .await
    {
        Ok(items) => AppWrite::<BlobTreeMsg>::ok(items),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
