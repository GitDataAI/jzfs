use actix_web::web::{get, post, scope};

pub mod merge;
pub mod repo;
pub mod users;
pub mod utils;
pub mod rstatic;
pub mod org;

pub fn v1_route(config: &mut actix_web::web::ServiceConfig) {
    config.service(
        scope("/v1")
            .route("", get().to(v1_hello))
            .route("/check/{name}", get().to(utils::check_name::check_name))
            .service(
                scope("/merge").route("/users/{username}", get().to(merge::users::merge_users)),
            )
            .service(
                scope("/users")
                    .route("/sigck", post().to(users::sigops::sigin_check_by_username))
                    .route("/sigin", post().to(users::sigops::sigin)) // must captcha
                    .route("/sigup", post().to(users::sigops::sigup)) // must captcha
                    .route("/sigout", post().to(users::sigops::sigout))
                    .route("/check", get().to(users::sigops::sigin_check))
                    .route("/profile",post().to(users::profile::update_profile::update_profile))
                    .route("/profile", get().to(users::profile::get_profile::get_profile))
                    .route("/orgs", post().to(org::member::list::org_owner_list))
                    .service(
                        scope("/{owner}")
                            .route("/repo", post().to(repo::list::repo_user_list))
                            .route("/orgs", post().to(org::member::list::org_user_list))
                    )
            )
            .service(
                scope("/context")
                    .route("/current", get().to(users::context::current_context))
                    .route("/list", get().to(users::context::list_context))
                    .route("/switch/{uid}", post().to(users::context::switch_context)),
            )
            .service(
                scope("/utils")
                    .route("/base64_captcha", get().to(utils::base64_captcha::utils_captcha))
                    .route("/email_captcha_send", post().to(utils::email_captcha::email_captcha))
                    .route("/email_captcha_check", post().to(utils::email_captcha::email_captcha_check)),
            )
            .service(
                scope("/repo")
                    .route("", post().to(repo::init::repo_init))
                    .route("/access", post().to(repo::init::repo_access))
                    .service(
                        scope("/{owner}/{repo}")
                            .route("", post().to(repo::info::repo_info))
                            .route("/blob", post().to(repo::fsops::blob::repo_blob))
                            .route("/star", post().to(repo::soc::star::repo_star))
                            .route("/watch", post().to(repo::soc::watch::repo_watch))
                            .route("/can_setting", get().to(repo::info::repo_can_setting))
                            .service(
                                scope("/branch")
                                    .route("/list", get().to(repo::fsops::branch::list_branch))
                                    .route("/create", post().to(repo::fsops::branch::create_branch))
                                    .route(
                                        "/delete/{name}",
                                        post().to(repo::fsops::branch::delete_branch),
                                    )
                                    .route(
                                        "/rename/{name}/{new_name}",
                                        post().to(repo::fsops::branch::rename_branch),
                                    )
                                    .route(
                                        "/checkout/{name}",
                                        post().to(repo::fsops::branch::checkout_head),
                                    ),
                            )
                            .service(
                                scope("/tree")
                                    .route("/list", post().to(repo::fsops::tree::repo_tree))
                                    .route(
                                        "/message",
                                        post().to(repo::fsops::tree::repo_tree_message),
                                    ),
                            ),
                    ),
            )
            .service(
                scope("/static")
                    .route("/upload_avatar", post().to(rstatic::avatar::upload_avatar))
                    .route("/img/{path}", get().to(rstatic::avatar::down_avatar))
            )
            .service(
                scope("/orgs")
                    .route("",post().to(org::create::create_org))
                    .service(
                        scope("/{org}")
                            .route("", get().to(org::info::org_info))
                            .route("/can_setting", get().to(org::info::org_can_setting))
                            .route("/member",post().to(org::member::list::org_member_list))
                    )
            )
    );
}

async fn v1_hello() -> String {
    format!(
        "hello, this v1 route, now: {}",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
    )
}
