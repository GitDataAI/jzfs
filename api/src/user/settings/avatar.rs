use crate::AppStatus;
use actix_web::Responder;
use actix_web::web::Bytes;
use error::AppResult;
use session::Session;

pub async fn api_setting_avatar_upload(
    session: Session,
    core: AppStatus,
    payload: Bytes,
) -> impl Responder {
    core.setting_avatar_upload(session, payload.to_vec())
        .await
        .into_response()
}
