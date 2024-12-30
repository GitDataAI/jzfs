use crate::api::app_writer::AppWrite;
use crate::api::handlers::user::AvatarGet;
use crate::api::middleware::session::SessionModel;
use crate::server::MetaData;
use actix_session::Session;
use actix_web::{web, Responder};

pub async fn avatar_get(session: Session, cfg: web::Data<MetaData>) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(err) => return AppWrite::<AvatarGet>::unauthorized(err.to_string()),
    };
    match cfg.users_avatar_get(model.uid).await {
        Ok(Some(avatar)) => {
            let inner = AvatarGet { url: avatar };
            AppWrite::ok(inner)
        }
        Ok(None) => AppWrite::ok(AvatarGet {
            url: "".to_string(),
        }),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

pub async fn avatar_set(
    session: Session,
    cfg: web::Data<MetaData>,
    inner: web::Json<AvatarGet>,
) -> impl Responder {
    let model = match SessionModel::authenticate(session).await {
        Ok(model) => model,
        Err(err) => return AppWrite::<AvatarGet>::unauthorized(err.to_string()),
    };
    match cfg.users_avatar_set(model.uid, inner.url.clone()).await {
        Ok(_unused) => AppWrite::ok(AvatarGet {
            url: inner.url.clone(),
        }),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
