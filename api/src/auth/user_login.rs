use crate::AppStatus;
use actix_web::Responder;
use actix_web::web::Json;
use core::auth::user_login::AuthUserLoginParam;
use error::AppResult;
use session::Session;

pub async fn api_auth_user_login(
    session: Session,
    payload: Json<AuthUserLoginParam>,
    core: AppStatus,
) -> impl Responder {
    core.auth_user_login(payload.clone(), session)
        .await
        .into_response()
}
