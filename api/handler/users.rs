use actix_session::Session;
use actix_web::{web, Responder};
use crate::api::write::AppWrite;
use crate::services::AppState;
use uuid::Uuid;
use crate::services::user::update::UserUpdateOptional;
use crate::model::users::users;
use crate::services::user::ssh_pubkey::SSHKeyCreateParma;
use crate::services::user::token::TokenDelete;

pub async fn user_now(
    session: Session,
    state: web::Data<AppState>,
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

pub async fn user_token_list(
    session: Session,
    state: web::Data<AppState>,
)
    -> impl Responder
{
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
    match state.token_list(uid).await{
        Ok(x) => {
            AppWrite::ok(x)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

pub async fn user_token_create(
    session: Session,
    state: web::Data<AppState>,
    form: web::Json<crate::services::user::token::TokenCreate>,
)
    -> impl Responder
{
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
    match state.token_create(uid, form.0).await{
        Ok(x) => {
            AppWrite::ok(x)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

pub async fn user_token_delete(
    session: Session,
    state: web::Data<AppState>,
    parma: web::Json<TokenDelete>,
)
    -> impl Responder
{
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
    match state.token_delete(uid, parma.into_inner()).await{
        Ok(x) => {
            AppWrite::ok(x)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

pub async fn user_ssh_key_list(
    session: Session,
    state: web::Data<AppState>,
)
    -> impl Responder
{
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
    match state.ssh_key_list(uid).await{
        Ok(x) => {
            AppWrite::ok(x)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

pub async fn user_ssh_key_create(
    session: Session,
    state: web::Data<AppState>,
    form: web::Json<SSHKeyCreateParma>,
)
    -> impl Responder
{
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
    match state.ssh_key_insert(uid, form.0).await{
        Ok(x) => {
            AppWrite::ok(x)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}

pub async fn user_ssh_key_delete(
    session: Session,
    state: web::Data<AppState>,
    parma: web::Path<Uuid>,
)
    -> impl Responder
{
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
    match state.ssh_key_delete(uid, parma.into_inner()).await{
        Ok(x) => {
            AppWrite::ok(x)
        },
        Err(err) => {
            AppWrite::error(err.to_string())
        }
    }
}
