use crate::AppStatus;
use actix_web::{Responder, web};
use error::AppResult;

pub async fn api_users_active(username: web::Path<String>, core: AppStatus) -> impl Responder {
    core.users_active(&username.into_inner())
        .await
        .into_response()
}
