use crate::AppStatus;
use actix_web::Responder;
use error::AppResult;
use session::Session;

pub async fn api_auth_user_logout(session: Session, core: AppStatus) -> impl Responder {
    core.auth_user_logout(session).await.into_response()
}
