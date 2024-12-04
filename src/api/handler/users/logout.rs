use actix_session::Session;
use actix_web::Responder;
use crate::utils::r::R;


#[utoipa::path(
    post,
    tag = "users",
    path = "/api/v1/users/logout",
    responses(
            (status = 200, description = "Logout Success"),
            (status = 401, description = "Not Login")
    ),
)]
pub async fn api_user_logout(
    session: Session
) -> impl Responder
{
    session.purge();
    R::<String>{
        code: 200,
        msg: Option::from("[Ok]".to_string()),
        data: None,
    }
}