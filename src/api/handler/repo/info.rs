use actix_web::{web, Responder};
use uuid::Uuid;
use crate::api::service::Service;
use crate::metadata::model::repos::repo;
use crate::utils::r::R;

#[utoipa::path(
    get,
    tag = "repos",
    path = "/api/v1/repo/{repo}/info",
    params(
        ("repo" = Uuid, description = "Repo Uid"),
    ),
    responses(
        (status = 200, description = "Repo found successfully"),
        (status = 400, description = "Repo Not Found"),
    ),
)]
pub async fn api_repo_info(
    repo: web::Path<Uuid>,
    service: web::Data<Service>
)
-> impl Responder
{
    let uid = repo.into_inner();
    match service.repo.info(uid).await{
        Ok(x) => R::<repo::Model>{
            code: 200,
            msg: Option::from("[Ok]".to_string()),
            data: Some(x)
        },
        Err(_) => R{
            code: 400,
            msg: Option::from("[Error] Repo NotFound".to_string()),
            data: None
        }
    }
}