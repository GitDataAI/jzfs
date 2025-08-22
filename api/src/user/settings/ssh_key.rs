use crate::AppStatus;
use actix_web::Responder;
use actix_web::web::{Json, Path, Query};
use core::Paginator;
use core::settings::sshkey::SettingSshKeyInsertParam;
use error::AppResult;
use session::Session;
use uuid::Uuid;

pub async fn api_user_setting_ssh_key_insert(
    core: AppStatus,
    session: Session,
    param: Json<SettingSshKeyInsertParam>,
) -> impl Responder {
    core.setting_ssh_key_insert(param.clone(), session)
        .await
        .into_response()
}

pub async fn api_user_setting_ssh_key_list(
    core: AppStatus,
    session: Session,
    paginator: Query<Paginator>,
) -> impl Responder {
    core.setting_ssh_key_list(session, paginator.0.clone())
        .await
        .into_response()
}

pub async fn api_user_setting_ssh_key_delete(
    core: AppStatus,
    session: Session,
    name: Path<Uuid>,
) -> impl Responder {
    core.setting_ssh_key_delete(session, name.into_inner())
        .await
        .into_response()
}
