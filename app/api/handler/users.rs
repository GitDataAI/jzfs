use actix_session::Session;
use actix_web::{web, Responder};
use crate::app::api::write::AppWrite;
use crate::app::services::AppState;
use uuid::Uuid;
use crate::app::services::user::update::UserUpdateOptional;
use crate::model::users::users;

pub async fn user_now(
    session: Session,
    state: web::Data<AppState>,
) -> impl Responder {
    session.renew();
    let uid = match session.get::<String>("user"){
        Ok(Some(uid)) => match serde_json::from_str::<users::Model>(&uid) {
            Ok(uid) => uid.uid,
            Err(_) => {
                return AppWrite::unauthorized("请先登录".to_string())
            }
        },
        Ok(None) => {
            return AppWrite::unauthorized("请先登录".to_string())
        },
        Err(_) => {
            return AppWrite::unauthorized("请先登录".to_string())
        }
    };

    match state.user_info_by_uid(uid).await{
        Ok(user) => {
            AppWrite::ok(user)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}


pub async fn user_dashbored(
    path: web::Path<(String,)>,
    state: web::Data<AppState>,
) -> impl Responder {
    let (username,) = path.into_inner();
    let user = match state.user_info_by_username(username).await {
        Ok(user) => user,
        Err(err) => {
            return AppWrite::error(err.to_string())
        }
    };
    match state.user_dashbored(user.uid).await{
        Ok(x) => {
            AppWrite::ok(x)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

pub async fn user_update_optional(
    session: Session,
    state: web::Data<AppState>,
    form: web::Json<UserUpdateOptional>,
) -> impl Responder {
    let uid = match session.get::<String>("user"){
        Ok(Some(uid)) => match serde_json::from_str::<users::Model>(&uid) {
            Ok(uid) => uid.uid,
            Err(_) => {
                return AppWrite::unauthorized("请先登录".to_string())
            }
        },
        Ok(None) => {
            return AppWrite::unauthorized("请先登录".to_string())
        },
        Err(_) => {
            return AppWrite::unauthorized("请先登录".to_string())
        }
    };
    match state.user_update_optional(uid, form.0).await{
        Ok(x) => {
            AppWrite::ok(x)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

pub async fn user_info_by_uid(
    state: web::Data<AppState>,
    uid: web::Path<Uuid>,
) -> impl Responder {
    match state.user_info_by_uid(uid.into_inner()).await{
        Ok(x) => {
            AppWrite::ok(x)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

