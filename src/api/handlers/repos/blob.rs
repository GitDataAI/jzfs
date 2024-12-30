use crate::api::app_writer::AppWrite;
use crate::api::handlers::repos::options::RepoPath;
use crate::server::MetaData;
use actix_web::web::{Data, Json, Path};
use actix_web::Responder;

pub async fn repo_latest_blob(
    path: Path<(String, String, String)>,
    meta: Data<MetaData>,
    query: Json<RepoPath>,
) -> impl Responder {
    match meta
        .repo_blob(
            path.0.clone(),
            path.1.clone(),
            path.2.clone(),
            None,
            query.path.clone(),
        )
        .await
    {
        Ok(items) => AppWrite::<Vec<u8>>::ok(items),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
pub async fn repo_blob_sha(
    path: Path<(String, String, String, String)>,
    meta: Data<MetaData>,
    query: Json<RepoPath>,
) -> impl Responder {
    match meta
        .repo_blob(
            path.0.clone(),
            path.1.clone(),
            path.2.clone(),
            Some(path.3.clone()),
            query.path.clone(),
        )
        .await
    {
        Ok(items) => AppWrite::<Vec<u8>>::ok(items),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
