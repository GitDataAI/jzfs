use crate::api::app_writer::AppWrite;
use crate::api::handlers::users::options::{Base64Inner, UsersApply};
use crate::server::MetaData;
use actix_web::{web, Responder};
use uuid::Uuid;

pub async fn apply(inner: web::Json<Base64Inner>, meta: web::Data<MetaData>) -> impl Responder {
    let inner = match inner.decode::<UsersApply>() {
        Ok(inner) => inner,
        Err(err) => return AppWrite::<Uuid>::fail(err.to_string()),
    };
    match meta
        .users_apply(inner.username, inner.password, inner.email)
        .await
    {
        Ok(user) => AppWrite::ok(user),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
