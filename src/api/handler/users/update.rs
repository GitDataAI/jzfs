use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::dto::users::UserUpdate;
use crate::api::middleware::session::{SessionModel, SESSION_USER_KEY};
use crate::api::service::Service;
use crate::utils::r::R;

#[utoipa::path(
    post,
    tag = "users",
    path = "/api/v1/users/update",
    request_body = UserUpdate,
    responses(
            (status = 200, description = "Upload Success"),
            (status = 400, description = "Other Error"),
            (status = 401, description = "User Not Login"),
    ),
)]
pub async fn api_user_update(
    session: Session,
    service: web::Data<Service>,
    dto: web::Json<UserUpdate>
)
    -> impl Responder
{
    let user = session.get::<SessionModel>(SESSION_USER_KEY).unwrap();
    if user.is_none() {
        return R::<String> {
            code: 401,
            msg: Option::from("[Error] User Not Login".to_string()),
            data: None,
        }
    }
    let user = user.unwrap();
    match service.users.update_by_uid(user.uid, dto.into_inner()).await {
        Ok(_info) => {
            R::<String> {
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: None,
            }
        },
        Err(e) => {
            R::<String> {
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}