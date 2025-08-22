use crate::{AppStatus, Paginator};
use actix_web::Responder;
use actix_web::web::Query;
use error::{AppError, AppResult};
use session::Session;

pub async fn api_repos_recommend(
    app: AppStatus,
    paginator: Query<Paginator>,
    session: Session,
) -> impl Responder {
    app.repo_vector_search(session, paginator.into_inner())
        .await
        .map_err(AppError::from)
        .into_response()
}
