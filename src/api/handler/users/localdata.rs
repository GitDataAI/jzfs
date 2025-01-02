use actix_session::Session;
use actix_web::Responder;
use crate::api::middleware::session::{SessionModel, SESSION_USER_KEY};
use crate::utils::r::R;

#[utoipa::path(
    get,
    tag = "users",
    path = "/api/v1/users/local",
    responses(
            (status = 200, description = "Data", body=SessionModel),
            (status = 401, description = "Not Login")
    ),
)]
pub async fn api_user_local(
    session: Session
)
    -> impl Responder{
    let model = session.get::<SessionModel>(SESSION_USER_KEY).unwrap().unwrap();
    R::<SessionModel>{
        code: 200,
        msg: Option::from("[Ok]".to_string()),
        data: Some(model),
    }
}