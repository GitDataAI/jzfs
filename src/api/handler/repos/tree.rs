use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::dto::repo_dto::RepoTree;
use crate::metadata::service::MetaService;


#[utoipa::path(
    get,
    path = "/api/repo/{owner}/{repo}/tree/{path}?{commit={refs}:?}",
    responses(
        (status = 200, description = "success", body = RepoTree),
        (status = 400, description = "fail", body = String),
    ),
    params(
        ("owner" = String, description = "owner uid"),
        ("repo" = String, description = "repo name"),
        ("path" = String, description = "path"),
    ),
    tag = "repo",
)]
pub async fn api_repo_tree(
    service: web::Data<MetaService>,
    path: web::Path<(String, String, String)>,
    query: web::Query<crate::api::dto::repo_dto::RepoTreeQuery>,
)
    -> impl Responder
{
    let (owner, repo, branch) = path.into_inner();
    let repo_id = match service.repo_service().owner_name_by_uid(owner, repo).await{
        Ok(repo_id) => repo_id,
        Err(e) => return AppWrite::<RepoTree>::fail(e.to_string())
    };
    match service.repo_service().tree(repo_id, branch, query.commit.clone()).await{
        Ok(tree) => AppWrite::ok(tree),
        Err(e) => AppWrite::fail(e.to_string())
    }
}
