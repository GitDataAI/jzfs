use actix_session::Session;
use actix_web::{web, Responder};
use uuid::Uuid;
use crate::api::dto::repo::RepoRename;
use crate::api::handler::grand::repo_owner::check_repo_owner;
use crate::api::service::Service;
use crate::utils::r::R;

#[utoipa::path(
    post,
    tag = "repo",
    path = "/api/repo/{repo_id}/rename",
    params(
        ("repo_id" = Uuid, description = "repo id"),
    ),
    request_body = RepoRename,
    responses(
        (status = 200, description = "success", body= bool),
        (status = 400, description = "error", body = String)
    )
)]
pub async fn api_repo_rename(
    service: web::Data<Service>,
    path: web::Path<Uuid>,
    dto: web::Json<RepoRename>,
    session: Session
)
-> impl Responder
{
    let session = service.check.check_session(session).await;
    if !session.is_err(){
        return R::<bool>{
            code: 400,
            data: Option::from(false),
            msg: Option::from("[Error] Not Permission".to_string())
        }
    }
    let repo_id = path.into_inner();
    let session = session.unwrap();
    let check = check_repo_owner(&service, session.uid, repo_id).await;
    if check.is_err(){
        return R::<bool>{
            code: 400,
            data: Option::from(false),
            msg: Option::from("[Error] Not Permission".to_string())
        }
    }
    if !check.unwrap(){
        return R::<bool>{
            code: 400,
            data: Option::from(false),
            msg: Option::from("[Error] Not Permission".to_string())
        }
    }
    match service.repo.rename(repo_id, dto.name.to_string()).await{
        Ok(_)=>{
            R::<bool>{
                code: 200,
                data: Option::from(true),
                msg: Option::from("[Ok]".to_string())
            }
        },
        Err(_)=>{
            R::<bool>{
                code: 400,
                data: Option::from(false),
                msg: Option::from("[Error]".to_string())
            }
        }
    }
    
    
}