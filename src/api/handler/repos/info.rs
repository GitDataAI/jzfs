use crate::api::app_write::AppWrite;
use crate::metadata::service::MetaService;
use actix_web::{web, Responder};


#[utoipa::path(
    get,
    tag = "repo",
    path = "/api/v1/repos/{owner}/{repo}",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request")
   )
)]
pub async fn api_repo_info_get(
    service: web::Data<MetaService>,
    path: web::Path<(String,String)>
) -> impl Responder
{
    
    let (owner, repo) = path.into_inner();
    tracing::info!("owner: {}, repo: {}", owner, repo);
    let repo_id = match service.repo_service().owner_name_by_uid(owner,repo).await{
        Ok(data) => {
            data
        }
        Err(e) => {
            return AppWrite::error(e.to_string())
        }
    };
    match service.repo_service().info(repo_id).await{
        Ok(data) => {
            AppWrite::ok(data)
        }
        Err(e) => {
            AppWrite::error(e.to_string())
        }
    }
}
