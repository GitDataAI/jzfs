use crate::AppStatus;
use actix_web::Responder;
use actix_web::web::Json;
use error::AppResult;
use session::Session;

pub async fn api_repo_init_owner_select(session: Session, core: AppStatus) -> impl Responder {
    core.repo_init_select_owner(session).await.into_response()
}

pub async fn api_repo_init_before(
    param: Json<core::repos::init::RepoInitBefore>,
    core: AppStatus,
) -> impl Responder {
    core.repo_init_before(param.into_inner())
        .await
        .into_response()
}

pub async fn api_repo_init(
    param: Json<core::repos::init::RepoInitParam>,
    core: AppStatus,
    session: Session,
) -> impl Responder {
    core.repo_init_main(param.into_inner(), session)
        .await
        .into_response()
}

pub async fn api_repo_init_storage(core: AppStatus) -> impl Responder {
    core.repo_init_select_storage().await.into_response()
}
