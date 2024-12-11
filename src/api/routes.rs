use actix_web::middleware::from_fn;
use actix_web::web::{delete, get, post, put};
use actix_web::web;
use crate::api::handler::email::captcha::{api_email_captcha_check, api_email_rand_captcha};
use crate::api::handler::email::forget::api_email_forget;
use crate::api::handler::group::creat::api_group_create;
use crate::api::handler::group::info::api_group_info;
use crate::api::handler::owner::avatar::api_owner_avatar;
use crate::api::handler::owner::group::api_owner_group;
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
use crate::api::handler::version::api_version;
use crate::api::scalar::ApiDoc;
use crate::api::handler::owner::email::api_owner_email;
use crate::api::handler::owner::followers::api_owner_follower;
use crate::api::handler::owner::keys::api_owner_keys;
use crate::api::handler::owner::repo::api_owner_repo;
use crate::api::handler::owner::setting::api_owner_setting;
use crate::api::handler::owner::team::api_owner_team;
use crate::api::handler::teams::byuser::api_team_by_user;
use crate::api::handler::users::avatar::{api_user_avatar_delete, api_user_avatar_upload};
use crate::api::handler::users::keys::{api_users_key_create, api_users_key_remove};
use crate::api::handler::users::setting::api_user_setting;
use crate::api::middleware::auth::must_login::must_login;

pub fn routes(cfg: &mut web::ServiceConfig) {
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
                            // Now User Info
                            // .route("/info",get().to(api_owner_info))
                            // Now User Team
                            .route("/team",get().to(api_owner_team))
                            // Now User Group
                            .route("/group",get().to(api_owner_group))
                            // Now User Email
                            .route("/email", get().to(api_owner_email))
                            // Now User Followers
                            .route("/followers", get().to(api_owner_follower))
                            // Now User Repo
                            .route("/repo", get().to(api_owner_repo))
                            .route("/setting", get().to(api_owner_setting))
                            // Now User Key
                            .route("/keys", get().to(api_owner_keys))
                            // Now User GPG_KEY
                            .route("/gpg", get().to(||async { "TODO" }))
                            // Now User Star
                            .route("/star", get().to(||async { "TODO" }))
                            // Now User Watch Repo
                            .route("/watch", get().to(||async { "TODO" }))
                            // Now User Avatar
                            .route("/avatar", get().to(api_owner_avatar))
                    )
                    .service(
                        // User APi Start
                        web::scope("/users")
                            // User Apply
                            .route("/apply", post().to(api_user_apply))
                            // User Login - now provide two type, but last time will provide more than
                            .service(
                                web::scope("/login")
                                    // use UserName Login
                                    .route("/name", post().to(api_users_login_name))
                                    // use Email Login
                                    .route("/email", post().to(api_users_login_email))
                            )
                            // User Logout -- never you other login it also success
                            .route("/logout", post().to(api_user_logout))
                            .service(
                                // this is user must login after api
                                web::scope("")
                                    .wrap(from_fn(must_login))
                                    // the user in now in db data
                                    .route("/local", get().to(api_user_local))
                                    // reset it password
                                    .route("/reset", post().to(api_user_reset_passwd_profile))
                            )
                            .service(
                                // key -- you can use it access ssh or git -- but it will after time will success -- it maybe is rsa or ed25519
                                web::scope("/key")
                                    .wrap(from_fn(must_login))
                                    // create key
                                    .route("/create", post().to(api_users_key_create))
                                    // delete key
                                    .route("/{uid}", delete().to(api_users_key_remove))
                            )
                            .service(
                                // gpg -- you can use it access ssh or git -- but it will after time will success -- it maybe is rsa or ed25519
                                web::scope("/gpg")
                                    .wrap(from_fn(must_login))
                                    // create gpg
                                    .route("/create", post().to(||async { "TODO" }))
                                    // delete gpg
                                    .route("/delete", delete().to(||async { "TODO" }))
                            )
                            .service(
                                // user avatar -- it will be use in user avatar
                                web::scope("/avatar")
                                    .wrap(from_fn(must_login))
                                    // upload avatar
                                    .route("/upload", put().to(api_user_avatar_upload))
                                    // delete avatar
                                    .route("/clear", delete().to(api_user_avatar_delete))
                            )
                            .service(
                                web::scope("/email")
                                    .wrap(from_fn(must_login))
                                    .route("/bind", post().to(||async { "TODO" }))
                                    .route("/unbind", delete().to(||async { "TODO" }))
                                    .route("/verify", post().to(||async { "TODO" }))
                            )
                            .service(
                                web::scope("/following")
                                    .wrap(from_fn(must_login))
                                    .route("/add", post().to(||async { "TODO" }))
                                    .route("/del", delete().to(||async { "TODO" }))
                            )
                            .service(
                                web::scope("/star")
                                    .wrap(from_fn(must_login))
                                    .route("/add", post().to(||async { "TODO" }))
                                    .route("/del", delete().to(||async { "TODO" }))
                            )
                            .service(
                                web::scope("/watch")
                                    .wrap(from_fn(must_login))
                                    .route("/add", post().to(||async { "TODO" }))
                                    .route("/del", delete().to(||async { "TODO" }))
                            )
                            .service(
                                web::scope("/repo")
                                    .wrap(from_fn(must_login))
                                    .route("/create", post().to(||async { "TODO" }))
                                    .route("/delete", delete().to(||async { "TODO" }))
                            )
                            .service(
                                web::scope("/setting")
                                    .wrap(from_fn(must_login))
                                    .default_service(post().to(api_user_setting))
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
                            .route("", get().to(||async { "TODO" }))
                            .route("/creat", post().to(api_group_create))
                            .service(
                                web::scope("/{group}")
                                    .route("",get().to(api_group_info))
                                    .route("", post().to(||async { "TODO" }))
                                    .route("", delete().to(||async { "TODO" }))
                                    .service(
                                        web::scope("/avatar")
                                            .route("", put().to(||async { "TODO" }))
                                            .route("", delete().to(||async { "TODO" }))
                                            .route("", get().to(||async { "TODO" }))
                                    )
                                    .route("/member", get().to(||async { "TODO" }))
                                    .route("/member/{user}", post().to(||async { "TODO" }))
                                    .route("/member/{user}", delete().to(||async { "TODO" }))
                                    .route("/member/{user}/role", post().to(||async { "TODO" }))
                                    .service(
                                        web::scope("/repo")
                                            .route("/", get().to(||async { "TODO" }))
                                            .route("/", post().to(||async { "TODO" }))
                                    )
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
                    .service(
                        web::scope("/notification")
                            .route("/get", get().to(||async { "TODO" }))
                            .route("/read", post().to(||async { "TODO" }))
                            .route("/new", get().to(||async { "TODO" }))
                            .service(
                                web::scope("/threads")
                                    .route("/{id}", get().to(||async { "TODO" }))
                                    .route("/{id}", post().to(||async { "TODO" }))
                            )
                    )
                
        )
    ;
}
