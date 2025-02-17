use crate::app::api::write::AppWrite;
use crate::app::services::AppState;
use poem::session::Session;
use poem::{handler, web, IntoResponse};
use crate::app::services::user::update::UserUpdateOptional;
use crate::model::users::users;

#[handler]
pub async fn user_now(
    session: &Session,
    state: web::Data<&AppState>,
) -> impl IntoResponse {
    let uid = match session.get::<String>("user"){
        Some(uid) => match serde_json::from_str::<users::Model>(&uid) {
            Ok(uid) => uid.uid,
            Err(_) => {
                return AppWrite::unauthorized("请先登录".to_string())
            }
        },
        None => {
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


#[handler]
pub async fn user_dashbored(
    path: web::Path<(String,)>,
    state: web::Data<&AppState>,
) -> impl IntoResponse {
    let (username,) = path.0;
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

#[handler]
pub async fn user_update_optional(
    session: &Session,
    state: web::Data<&AppState>,
    form: web::Json<UserUpdateOptional>,
) -> impl IntoResponse {
    let uid = match session.get::<String>("user"){
        Some(uid) => match serde_json::from_str::<users::Model>(&uid) {
            Ok(uid) => uid.uid,
            Err(_) => {
                return AppWrite::unauthorized("请先登录".to_string())
            }
        },
        None => {
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

