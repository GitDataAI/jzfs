use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::dto::repo_dto::RepoCreate;
use crate::api::handler::check_session;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "groups",
    path = "/api/v1/groups/{group}/repo",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_groups_repo(
    service: web::Data<MetaService>,
    group: web::Path<String>
) -> impl Responder
{
    let groups = service.group_service().name_to_uid(group.into_inner()).await;
    if groups.is_err(){
        return AppWrite::fail("Not Found".to_string());
    }
    let models = service.group_service().owner_repo(groups.unwrap()).await;
    match models{
        Ok(models) => {
            AppWrite::ok(models)
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    post,
    tag = "groups",
    path = "/api/v1/groups/{group}/repo",
    params(
        ("group" = String, Path, description = "Group Name"),
    ),
    request_body = RepoCreate,
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_groups_repo_create(
    session: Session,
    service: web::Data<MetaService>,
    repo: web::Json<RepoCreate>,
    group: web::Path<String>
)
    -> impl Responder
{
    let uid = match check_session(session).await{
        Ok(x) => x.uid,
        Err(e) => return AppWrite::<String>::unauthorized(e.to_string())
    };
    let group_id = match service.group_service().name_to_uid(group.into_inner()).await{
        Ok(x) => x,
        Err(e) => return AppWrite::fail(e.to_string())
    };
    let mut repo = repo.into_inner();
    repo.owner = group_id;
    repo.is_group = true;
    match service.repo_service().create_repo(repo, uid).await{
        Ok(_) => AppWrite::ok("success".to_string()),
        Err(e) => AppWrite::fail(e.to_string())
    }
}