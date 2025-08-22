use crate::AppStatus;
use actix_web::{Responder, web};
use error::AppResult;
use session::Session;

pub async fn api_repos_refs_list(
    path: web::Path<(String, String)>,
    core: AppStatus,
) -> impl Responder {
    let (namespace, repo_name) = path.into_inner();
    core.repos_branch_list(&namespace, &repo_name)
        .await
        .into_response()
}

// pub async fn api_repos_refs_rename(
//     path: web::Path<(String, String, String)>,
//     core: AppStatus,
// ) -> impl Responder {
//     let (namespace, repo_name, refs_name) = path.into_inner();
//     core.repos_refs_rename(&namespace, &repo_name, &refs_name).await.into_response()
// }
pub async fn api_repos_refs_delete(
    path: web::Path<(String, String, String)>,
    core: AppStatus,
    session: Session,
) -> impl Responder {
    let (namespace, repo_name, refs_name) = path.into_inner();
    core.repo_branch_delete(&namespace, &repo_name, &refs_name, session)
        .await
        .into_response()
}
