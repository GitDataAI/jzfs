use crate::utils::r::R;
use actix_web::Responder;
use actix_web::web;
use uuid::Uuid;
use crate::api::service::Service;

#[utoipa::path(
    get,
    tag = "repos",
    path = "/api/v1/repo/{repo}/branch",
    responses(
        (status = 200, description = "Repo found successfully"),
        (status = 400, description = "Repo Not Found"),
    ),
)]
pub async fn api_repo_branch(
    repo: web::Path<Uuid>,
    service: web::Data<Service>
)
    -> impl Responder
{
    let uid = repo.into_inner();
    match service.repo.branch(uid).await{
        Ok(x) => R::<Vec<String>>{
            code: 200,
            msg: Option::from("[Ok]".to_string()),
            data: Some(x)
        },
        Err(_) => R{
            code: 400,
            msg: Option::from("[Error] Repo Not Found".to_string()),
            data: None
        }
    }
}
