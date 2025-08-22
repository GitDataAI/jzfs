use crate::AppStatus;
use actix_web::{Responder, web};
use error::AppResult;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OptionalRefSha {
    pub sha: Option<String>,
}

pub async fn api_repos_tree(
    path: web::Path<(String, String, String, String)>,
    core: AppStatus,
    param: web::Query<OptionalRefSha>,
) -> impl Responder {
    let (namespace, repo_name, refs, path) = path.into_inner();
    core.repos_tree(&namespace, &repo_name, Some(refs), param.sha.clone(), path)
        .await
        .into_response()
}
