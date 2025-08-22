use crate::{AppStatus, Paginator};
use actix_web::web::{Json, Query};
use actix_web::{Responder, web};
use error::AppResult;
use session::Session;

pub async fn api_user_setting_access_key_list(
    core: AppStatus,
    session: Session,
    paginator: Query<Paginator>,
) -> impl Responder {
    core.setting_access_key_list(session, paginator.0.clone())
        .await
        .into_response()
}

pub async fn api_user_setting_access_key_insert(
    core: AppStatus,
    session: Session,
    param: Json<core::settings::access_key::SettingAccessNewParam>,
) -> impl Responder {
    core.setting_access_key_new(session, param.clone())
        .await
        .into_response()
}

pub async fn api_user_setting_access_key_delete(
    core: AppStatus,
    session: Session,
    name: web::Path<String>,
) -> impl Responder {
    core.setting_access_key_delete(session, name.as_str())
        .await
        .into_response()
}
