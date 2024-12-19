use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::metadata::model::repo::repo_branch::Model;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "repo",
    path = "/api/repos/{owner}/{repo}/branchs",
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
pub async fn api_repo_branchs(
    service: web::Data<MetaService>,
    path: web::Path<(String, String)>,
) -> impl Responder
{
    let (owner, repo) = path.into_inner();
    match service.repo_service().branch(owner, repo).await {
        Ok(models) => {
            AppWrite::<Vec<Model>>::ok(models)
        },
        Err(err) => {
            AppWrite::fail(err.to_string())
        }
    }
}


#[utoipa::path(
    get,
    path = "/api/repos/{owner}/{repo}/branchs/{branch}",
    responses(
        (status = 200, description = "Get repo branch commits"),
        (status = 400, description = "Bad request", body = String),
        (status = 500, description = "Internal server error", body = String),
    ),
    params(
        ("owner" = String, description = "Repo owner"),
        ("repo"= String, description= "Repo name"),
    )
)]

pub async fn api_repo_branch(
    service: web::Data<MetaService>,
    path: web::Path<(String, String, String)>,
) -> impl Responder
{
    let (owner, repo, branch) = path.into_inner();
    match service.repo_service().branch_by_name(owner, repo, branch).await {
        Ok(models) => {
            AppWrite::<Model>::ok(models)
        },
        Err(err) => {
            AppWrite::fail(err.to_string())
        }
    }
}