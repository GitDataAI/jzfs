use crate::api::handler::email::captcha::{api_email_captcha_check, api_email_rand_captcha};
use crate::api::handler::email::forget::api_email_forget;
use crate::api::handler::group::creat::api_group_create;
use crate::api::handler::group::info::api_group_info;
use crate::api::handler::users::apply::api_user_apply;
use crate::api::handler::users::localdata::api_user_local;
use crate::api::handler::users::login::{api_users_login_email, api_users_login_name};
use crate::api::handler::users::logout::api_user_logout;
use crate::api::handler::users::reset::api_user_reset_passwd_profile;
use crate::api::handler::users::update::api_user_update;
use crate::api::handler::version::api_version;
use actix_web::web;
use actix_web::web::{get, post};

pub fn routes(cfg: &mut web::ServiceConfig){
    let start = std::time::Instant::now();
    cfg
        .app_data(web::Data::new(start))
        .route("/version", get().to(api_version))
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/users")
                            .route("/apply", post().to(api_user_apply))
                            .service(
                                web::scope("/login")
                                    .route("/name", post().to(api_users_login_name))
                                    .route("/email", post().to(api_users_login_email))
                            )
                            .route("/logout", post().to(api_user_logout))
                            .route("/local", post().to(api_user_local))
                            .route("/reset", post().to(api_user_reset_passwd_profile))
                            .route("/update",post().to(api_user_update))
                    )
                    .service(
                        web::scope("/email")
                            .service(
                                web::scope("/captcha")
                                    .route("/send", post().to(api_email_rand_captcha))
                                    .route("/verify", post().to(api_email_captcha_check))
                            )
                            .route("/forget",post().to(api_email_forget))
                    )
                    .service(
                        web::scope("/group")
                            .route("/creat", post().to(api_group_create))
                            .service(
                                web::scope("/{group}")
                                    .route("/info",post().to(api_group_info))
                            )
                    )
        )
    ;
}