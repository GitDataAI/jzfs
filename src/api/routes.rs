use actix_web::middleware::from_fn;
use crate::api::handler::email::captcha::{api_email_captcha_check, api_email_rand_captcha};
use crate::api::handler::email::forget::api_email_forget;
use crate::api::handler::group::creat::api_group_create;
use crate::api::handler::group::info::api_group_info;
use crate::api::handler::owner::group::api_owner_group;
use crate::api::handler::owner::info::api_owner_info;
use crate::api::handler::owner::team::api_owner_teams;
use crate::api::handler::repo::create::api_repo_create;
use crate::api::handler::teams::create::api_teams_create;
use crate::api::handler::teams::info::api_team_info;
use crate::api::handler::teams::invite::api_team_group_invite;
use crate::api::handler::teams::list::api_list_team;
use crate::api::handler::users::apply::api_user_apply;
use crate::api::handler::users::localdata::api_user_local;
use crate::api::handler::users::login::{api_users_login_email, api_users_login_name};
use crate::api::handler::users::logout::api_user_logout;
use crate::api::handler::users::reset::api_user_reset_passwd_profile;
use crate::api::handler::users::update::api_user_update;
use crate::api::handler::version::api_version;
use crate::api::scalar::ApiDoc;
use actix_web::web::{delete, get, post, put};
use actix_web::web;
use crate::api::handler::teams::byuser::api_team_by_user;
use crate::api::middleware::auth::must_login::must_login;

pub fn routes(cfg: &mut web::ServiceConfig){
    let start = std::time::Instant::now();
    cfg
        .app_data(web::Data::new(start))
        .service(ApiDoc::init())
        .route("/version", get().to(api_version))
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/owner")
                            .wrap(from_fn(must_login))
                            .route("/info",get().to(api_owner_info))
                            .route("/team",get().to(api_owner_teams))
                            .route("/group",get().to(api_owner_group))
                            .route("/email", get().to(||async { "TODO" }))
                            .route("/followers", get().to(||async { "TODO" }))
                            .route("/repo", get().to(||async { "TODO" }))
                            .route("/key", get().to(||async { "TODO" }))
                            .route("/gpg", get().to(||async { "TODO" }))
                            .route("/star", get().to(||async { "TODO" }))
                            .route("/watch", get().to(||async { "TODO" }))
                            .route("/avatar", get().to(||async { "TODO" }))
                    )
                    .service(
                        web::scope("/users")
                            .route("/apply", post().to(api_user_apply))
                            .service(
                                web::scope("/login")
                                    .route("/name", post().to(api_users_login_name))
                                    .route("/email", post().to(api_users_login_email))
                            )
                            .route("/logout", post().to(api_user_logout))
                            .service(
                                web::scope("")
                                    .wrap(from_fn(must_login))
                                    .route("/local", get().to(api_user_local))
                                    .route("/reset", post().to(api_user_reset_passwd_profile))
                                    .route("/update",post().to(api_user_update))
                            )
                            .service(
                                web::scope("/key")
                                    .route("/create", post().to(||async { "TODO" }))
                                    .route("/delete", delete().to(||async { "TODO" }))
                            )
                            .service(
                                web::scope("/gpg")
                                    .route("/create", post().to(||async { "TODO" }))
                                    .route("/delete", delete().to(||async { "TODO" }))
                            )
                            .service(
                                web::scope("/avatar")
                                    .route("/upload", put().to(||async { "TODO" }))
                            )
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
                                    .route("/info",get().to(api_group_info))
                            )
                    )
                    .service(
                        web::scope("/repo")
                            .route("/create",post().to(api_repo_create))
                    )
                    .service(
                        web::scope("/team")
                            .route("/list",get().to(api_list_team))
                            .route("/{group}/create",post().to(api_teams_create))
                            .route("/{group}/{team}/invite", post().to(api_team_group_invite))
                            .route("/{uid}/info", post().to(api_team_info))
                            .route("/byuser", get().to(api_team_by_user))
                    )
                
        )
    ;
}
