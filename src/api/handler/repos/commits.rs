use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::metadata::model::repo::repo_commit::Model;
use crate::metadata::service::MetaService;


#[utoipa::path(
    get,
    tag = "repo",
    path = "/api/repos/{owner}/{repo}/commits/{branch}",
    responses(
        (status = 200, description = "Get repo branchs"),
        (status = 400, description = "Bad request", body = String),
        (status = 500, description = "Internal server error", body = String),
    ),
    params(
        ("owner" = String, description = "Repo owner"),
        ("repo"= String, description = "Repo name"),
    )
)]
pub async fn api_repo_commits(
    service: web::Data<MetaService>,
    path: web::Path<(String, String, String)>,
) -> impl Responder
{
    let (owner, repo, branch) = path.into_inner();
    match service.repo_service().commits(owner, repo, branch).await {
        Ok(model) => {
            AppWrite::<Vec<Model>>::ok(model)
        },
        Err(err) => {
            AppWrite::fail(err.to_string())
        }
    }
}

#[utoipa::path(
    get,
    tag = "repo",
    path = "/api/repos/{owner}/{repo}/commits/{branch}/{commit_id}",
    responses(
        (status = 200, description = "Get repo branchs"),
        (status = 400, description = "Bad request", body = String),
        (status = 500, description = "Internal server error", body = String),
    ),
    params(
        ("owner" = String, description = "Repo owner"),
       ("repo"= String, description = "Repo name"),
    )
)]
pub async fn api_repo_commit(
    service: web::Data<MetaService>,
    path: web::Path<(String, String, String, String)>,
) -> impl Responder
{
    let (owner, repo, branch, commit_id) = path.into_inner();
    match service.repo_service().commit(owner, repo, branch, commit_id).await {
        Ok(model) => {
            AppWrite::ok(model)
        },
        Err(err) => {
            AppWrite::fail(err.to_string())
        }
    }
}