use actix_session::Session;
use actix_web::Responder;
use crate::api::app_write::AppWrite;
use crate::api::handler::check_session;
use crate::api::middleware::session::model::SessionModel;

pub async fn api_user_session_model(
    session: Session
)
-> impl Responder
{
    let model = check_session(session).await;
    if model.is_err() {
        AppWrite::<SessionModel>::unauthorized(model.err().unwrap().to_string())
    } else {
        AppWrite::ok(model.unwrap())
    }
}