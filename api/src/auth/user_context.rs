use crate::AppStatus;
use actix_web::Responder;
use error::AppResult;
use session::Session;

pub async fn api_auth_user_context(session: Session, core: AppStatus) -> impl Responder {
    core.user_context_current(session).await.into_response()
}
