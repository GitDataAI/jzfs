use crate::AppStatus;
use actix_web::{Responder, web};
use error::AppResult;
use session::Session;

pub async fn api_repo_data(
    session: Session,
    path: web::Path<(String, String)>,
    core: AppStatus,
) -> impl Responder {
    let (namespace, repo_name) = path.into_inner();
    core.repos_data(&namespace, &repo_name, session)
        .await
        .into_response()
}
