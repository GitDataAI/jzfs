use crate::api::service::Service;
use crate::store::dto::CommitDto;
use crate::utils::r::R;
use actix_web::{web, Responder};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/repo/{repo_id}/commit/{commit_id}/history",
    tag = "repo",
    params(
        ("repo_id" = String, description = "repo id"),
        ("commit_id" = String, description = "commit id"),
    ),
    responses(
        (status = 200, description = "success", body = Vec<CommitDto>),
        (status = 500, description = "error", body = String)
    )
)]
pub async fn api_repo_commit_history(
    path: web::Path<(Uuid,String)>,
    service: web::Data<Service>
)
-> impl Responder
{
    let (repo_id, commit_id) = path.into_inner();
    match service.repo.commit_history(repo_id, commit_id){
        Ok(commits)=>{
            R::<Vec<CommitDto>>{
                code: 200,
                data: Option::from(commits),
                msg: Option::from("success".to_string())
            }
        },
        Err(e)=>{
            R::<Vec<CommitDto>>{
                code: 500,
                data: None,
                msg: Option::from(e.to_string())
            }
        }
    }      
}