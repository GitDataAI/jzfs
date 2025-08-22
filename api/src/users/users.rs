use crate::{AppStatus, Paginator};
use actix_web::web::Query;
use actix_web::{Responder, web};
use error::{AppError, AppResult};
use session::Session;

pub async fn api_users(
    app: AppStatus,
    session: Session,
    username: web::Path<String>,
) -> impl Responder {
    app.users(&username.into_inner(), session)
        .await
        .into_response()
}

pub async fn api_users_star(
    app: AppStatus,
    username: web::Path<String>,
    paginator: Query<Paginator>,
) -> impl Responder {
    app.users_star_repos(&username.into_inner(), paginator.into_inner())
        .await
        .map_err(AppError::from)
        .into_response()
}
pub async fn api_users_watch(
    app: AppStatus,
    username: web::Path<String>,
    paginator: Query<Paginator>,
) -> impl Responder {
    app.users_watch_repos(&username.into_inner(), paginator.into_inner())
        .await
        .map_err(AppError::from)
        .into_response()
}

pub async fn api_users_repos(
    session: Session,
    app: AppStatus,
    paginator: Query<Paginator>,
    username: web::Path<String>,
) -> impl Responder {
    app.repo_find_by_owner(username.as_ref(), session, paginator.into_inner())
        .await
        .map_err(AppError::from)
        .into_response()
}
