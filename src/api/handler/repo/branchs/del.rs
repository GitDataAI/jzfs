use actix_web::Responder;
use crate::api::handler::grand::repo_access::check_repo_access;
use actix_session::Session;
use actix_web::web;
use uuid::Uuid;
use crate::api::dto::repo::RepoBranchDel;
use crate::api::service::Service;
use crate::utils::r::R;

#[utoipa::path(
    delete,
    tag = "repos",
    path = "/api/v1/repo/{repo}/branch/del",
    request_body = RepoBranchDel,
    responses(
        (status = 200, description = "Repo found successfully"),
        (status = 400, description = "Repo Not Found"),
        (status = 401, description = "Unauthorized")
    ),
)]
pub async fn api_repo_branch_del(
    repo: web::Path<Uuid>,
    service: web::Data<Service>,
    branch: web::Data<RepoBranchDel>,
    session: Session,
)
    -> impl Responder
{
    let session = service.check.check_session(session).await;
    if !session.is_ok() {
        return R {
            code: 401,
            data: None,
            msg: Option::from("unauthorized".to_string()),
        };
    }
    let repo_id = repo.into_inner();
    let check = check_repo_access(service.get_ref(),session.unwrap().uid, repo_id).await;
    if !check.is_ok(){
        return R {
            code: 401,
            data: None,
            msg: Option::from("unauthorized".to_string()),
        };
    }
    if check.unwrap() != true{
        return R {
            code: 401,
            data: None,
            msg: Option::from("unauthorized".to_string()),
        };
    }

    let branch = branch.into_inner();
    match service.repo.del_branch(repo_id, branch.branch.to_string()).await{
        Ok(_) => R::<String>{
            code: 200,
            msg: Option::from("[Ok]".to_string()),
            data: None
        },
        Err(_) => R{
            code: 400,
            msg: Option::from("[Error] Repo Not Found".to_string()),
            data: None
        }
    }
}