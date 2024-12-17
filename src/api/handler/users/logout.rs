use actix_session::Session;
use actix_web::Responder;
use crate::api::app_write::AppWrite;

#[utoipa::path(
    post,
    tag = "users",
    path = "/api/v1/users/logout",
    responses(
            (status = 200, description = "Logout Success"),
            (status = 401, description = "Not Login")
    ),
)]
pub async fn api_users_logout(
    session: Session
) -> impl Responder
{
    session.purge();
    AppWrite::<String>::ok_msg("ok".to_string())
}