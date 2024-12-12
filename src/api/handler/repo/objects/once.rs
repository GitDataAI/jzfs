use actix_web::web;
use uuid::Uuid;
use crate::api::dto::repo::RepoFilePath;
use crate::api::service::Service;
use crate::utils::r::R;

#[utoipa::path(
    get,
    tag = "repo",
    path = "/api/repo/{repo_id}/object/{branch}/{ref}/once",
    params(
        ("repo_id" = String, description = "repo id"),
        ("branch" = String, description = "branch name"),
        ("hash" = String, description = "hash"),
    ),
    request_body = RepoFilePath,
    responses(
        (status = 200, description = "success", body= Vec<u8>),
        (status = 500, description = "error", body = String)
    )
)]
pub async fn api_repo_object_once(
    service: web::Data<Service>,
    path: web::Path<(Uuid,String,String)>,
    dto: web::Json<RepoFilePath>
) -> impl actix_web::Responder
{
    let (repo_id, branch, hash) = path.into_inner();
    match service.repo.once_files(repo_id, branch, hash, dto.path.to_string()).await{
        Ok(byte)=>{
            R::<Vec<u8>>{
                code: 200,
                data: Option::from(byte),
                msg: Option::from("success".to_string())
            }
        },
        Err(e)=>{
            R{
                code: 500,
                data: None,
                msg: Option::from(e.to_string())
            }
        }
    }
}
