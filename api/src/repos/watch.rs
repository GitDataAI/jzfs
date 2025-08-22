use crate::AppCore;
use actix_web::{Responder, web};
use error::{AppError, AppResult};
use session::Session;

pub async fn api_repos_watch_repo(
    core: web::Data<AppCore>,
    session: Session,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (owner, repo) = path.into_inner();
    core.watch_repo(&owner, &repo, session)
        .await
        .map_err(AppError::from)
        .into_response()
}

pub async fn api_repos_unwatch_repo(
    core: web::Data<AppCore>,
    session: Session,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (owner, repo) = path.into_inner();
    core.unwatch_repo(&owner, &repo, session)
        .await
        .map_err(AppError::from)
        .into_response()
}
