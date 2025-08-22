use crate::AppStatus;
use actix_web::Responder;
use actix_web::web::Json;
use error::AppResult;
use session::Session;

pub async fn api_setting_basic_get(core: AppStatus, session: Session) -> impl Responder {
    core.get_user_basic_setting(session).await.into_response()
}

pub async fn api_setting_basic(
    core: AppStatus,
    session: Session,
    param: Json<crate::settings::basic_form::SettingBasicFormParam>,
) -> impl Responder {
    core.setting_basic_form_update(param.clone(), session)
        .await
        .into_response()
}
