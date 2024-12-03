use actix_session::Session;
use actix_web::Responder;
use crate::utils::r::R;

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