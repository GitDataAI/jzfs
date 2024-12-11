use actix_web::{web, Responder};
use uuid::Uuid;
use crate::api::service::Service;
use crate::store::dto::ObjectFile;
use crate::utils::r::R;

#[utoipa::path(
    get,
    tag = "repos",
    path = "/api/v1/repo/tree/{repo}/{branch}",
    responses(
        (status = 200, description = "Ok", body = Vec<ObjectFile>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Other Error"),
    ),
)]
pub async fn api_repo_object_tree(
    service: web::Data<Service>,
    rb: web::Path<(Uuid, Uuid)>,
) -> impl Responder 
{
    let (repo, branch) = rb.into_inner();
    let repo = service.repo.transaction.object_tree(repo, branch).await;
    match repo {
        Ok(x) => R::<Vec<ObjectFile>>{
            code: 200,
            data: Some(x),
            msg: Option::from("[Success] Get Object Tree Success".to_string()),
        },
        Err(e) => R {
            code: 500,
            data: None,
            msg: Option::from(format!("[Error] {}", e.to_string())),
        },
    }
}