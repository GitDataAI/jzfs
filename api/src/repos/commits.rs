use crate::{AppStatus, Paginator};
use actix_web::Responder;
use actix_web::web::{Path, Query};
use error::AppResult;

pub async fn api_repos_commit_list(
    path: Path<(String, String, String)>,
    core: AppStatus,
    query: Query<Paginator>,
) -> impl Responder {
    let (namespace, repo_name, branch) = path.into_inner();
    core.repos_commit_list(&namespace, &repo_name, &branch, query.into_inner())
        .await
        .into_response()
}
