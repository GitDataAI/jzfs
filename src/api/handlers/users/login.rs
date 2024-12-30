use crate::api::app_writer::AppWrite;
use crate::api::handlers::users::options::{Base64Inner, UsersLogin};
use crate::api::middleware::session::SessionModel;
use crate::server::MetaData;
use actix_session::Session;
use actix_web::{web, Responder};

pub async fn session(session: Session, meta: web::Data<MetaData>) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(err) => return AppWrite::<SessionModel>::unauthorized(err.to_string()),
    };
    match meta.users_info_uid(model.uid).await {
        Ok(model)=> AppWrite::ok(SessionModel::from(&model)),
        Err(err)=>AppWrite::fail(err.to_string()),
    }
}

pub async fn login(
    session: Session,
    inner: web::Json<Base64Inner>,
    meta: web::Data<MetaData>,
) -> impl Responder {
    let inner = match inner.decode::<UsersLogin>() {
        Ok(inner) => inner,
        Err(err) => return AppWrite::<SessionModel>::fail(err.to_string()),
    };
    match meta.users_login(inner.username, inner.password).await {
        Ok(user) => {
            let model = SessionModel::from(&user);
            model.insert(session).await;
            AppWrite::ok(model)
        }
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

pub async fn logout(session: Session) -> impl Responder {
    session.clear();
    AppWrite::ok("ok".to_string())
}
