use crate::api::app_writer::AppWrite;
use crate::api::handlers::repos::options::QueryList;
use crate::models::repos::commits;
use crate::server::MetaData;
use actix_web::web::{Data, Path, Query};
use actix_web::Responder;

pub async fn repo_commit_list(
    path: Path<(String, String, String)>,
    meta: Data<MetaData>,
    query: Query<QueryList>,
) -> impl Responder {
    match meta
        .repo_commits(
            path.0.clone(),
            path.1.clone(),
            path.2.clone(),
            query.page,
            query.size,
        )
        .await
    {
        Ok(items) => AppWrite::<Vec<commits::Model>>::ok(items),
        Err(e) => AppWrite::error(e.to_string()),
    }
}

pub async fn repo_commit_sha(
    path: Path<(String, String, String, String)>,
    meta: Data<MetaData>,
) -> impl Responder {
    match meta
        .repo_commit_sha(
            path.0.clone(),
            path.1.clone(),
            path.2.clone(),
            path.3.clone(),
        )
        .await
    {
        Ok(model) => AppWrite::<commits::Model>::ok(model),
        Err(e) => AppWrite::error(e.to_string()),
    }
}
