use crate::AppCore;
use actix_web::{Responder, web};
use error::{AppError, AppResult};
use session::Session;

pub async fn api_repos_star_repo(
    core: web::Data<AppCore>,
    session: Session,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (owner, repo) = path.into_inner();
    core.star_repo(&owner, &repo, session)
        .await
        .map_err(AppError::from)
        .into_response()
}

pub async fn api_repos_unstar_repo(
    core: web::Data<AppCore>,
    session: Session,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (owner, repo) = path.into_inner();
    core.unstar_repo(&owner, &repo, session)
        .await
        .map_err(AppError::from)
        .into_response()
}
