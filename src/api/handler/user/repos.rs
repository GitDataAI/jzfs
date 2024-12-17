use crate::api::app_write::AppWrite;
use crate::api::dto::repo_dto::RepoCreate;
use crate::api::handler::check_session;
use crate::metadata::model::repo::repo::Model;
use crate::metadata::service::MetaService;
use actix_session::Session;
use actix_web::{web, Responder};

#[utoipa::path(
    get,
    tag = "user",
    path = "/api/v1/user/repo",
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_repo(
    session: Session,
    service: web::Data<MetaService>,
) -> impl Responder 
{
    let session = check_session(session).await;
    if !session.is_ok() {
        return AppWrite::<Vec<Model>>::unauthorized(session.err().unwrap().to_string())
    }
    let session = session.unwrap();
    match service.user_service().owner_repo(session.uid).await {
        Ok(x) => {
            AppWrite::ok(x)
        },
        Err(e) => {
            AppWrite::fail(e.to_string())
        }
    }
}

#[utoipa::path(
    post,
    tag = "user",
    path = "/api/v1/user/repo",
    request_body(content = RepoCreate, description = "Create Repo", content_type = "application/json"),
    responses(
        (status = 200, description = "Success"),
        (status = 401, description = "Not Login")
    )
)]
pub async fn api_user_repo_create(
    session: Session,
    service: web::Data<MetaService>,
    repo: web::Json<RepoCreate>
) 
    -> impl Responder
{
    let uid = match check_session(session).await{
        Ok(x) => x.uid,
        Err(e) => return AppWrite::<String>::unauthorized(e.to_string())
    };
    match service.repo_service().create_repo( repo.into_inner(), uid).await{
        Ok(_) => AppWrite::ok("success".to_string()),
        Err(e) => AppWrite::fail(e.to_string())
    }
}