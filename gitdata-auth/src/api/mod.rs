use actix_session::Session;
use actix_web::web;
use actix_web::web::get;
use actix_web::web::post;
use lib_entity::sqlx::types::chrono::Utc;

use crate::api::apply::auth_apply;
use crate::api::captcha::auth_captcha_email_check;
use crate::api::captcha::auth_captcha_email_send;
use crate::api::captcha::auth_captcha_image;
use crate::api::check::auth_check;
use crate::api::login::auth_password;

pub mod apply;
pub mod captcha;
pub mod check;
pub mod login;
pub mod now;

pub fn router(cfg : &mut web::ServiceConfig) {
    cfg.route("/auth/login", post().to(auth_password))
        .route("/auth/logout", get().to(now::auth_now_logout))
        .route("/auth/captcha/image", post().to(auth_captcha_image))
        .route("/auth/apply", post().to(auth_apply))
        .route("/auth/check", post().to(auth_check))
        .route("/auth/index", get().to(index))
        .route("/auth/email/send", post().to(auth_captcha_email_send))
        .route("/auth/email/check", post().to(auth_captcha_email_check))
        .route("/auth/now/session", get().to(now::auth_now_session))
        .route("/auth/now/users", get().to(now::auth_now_users));
}

async fn index(session : Session) -> String {
    session.insert("index", Utc::now().to_rfc2822()).ok();
    "Hello Auth Serve".to_string()
}
