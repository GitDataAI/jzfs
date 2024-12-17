use actix_web::web;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use crate::api::app_docs::ApiDoc;
use crate::api::handler::groups::avatar::{api_groups_avatar, api_groups_avatar_upload};
use crate::api::handler::groups::create::api_groups_create;
use crate::api::handler::groups::info::api_groups_info;
use crate::api::handler::groups::labels::{api_groups_labels, api_groups_labels_create, api_groups_labels_delete, api_groups_labels_update};
use crate::api::handler::groups::members::{api_groups_member_add, api_groups_member_remove, api_groups_members, api_user_groups};
use crate::api::handler::groups::repos::{api_groups_repo, api_groups_repo_create};
use crate::api::handler::groups::search::api_groups_search;
use crate::api::handler::user::avatar::{api_user_avatar, api_user_avatar_delete, api_user_avatar_upload};
use crate::api::handler::user::emails::{api_user_email, api_user_email_bind, api_user_email_unbind};
use crate::api::handler::user::follower::{api_user_follow, api_user_followed, api_user_follower, api_user_unfollow};
use crate::api::handler::user::keys::{api_use_key_once, api_user_key_create, api_user_key_remove, api_user_keys};
use crate::api::handler::user::repos::{api_user_repo, api_user_repo_create};
use crate::api::handler::user::setting::{api_user_setting_get, api_user_setting_patch};
use crate::api::handler::user::starred::{api_user_star_add, api_user_star_remove, api_user_staring};
use crate::api::handler::user::subscriptions::{api_user_subscription_add, api_user_subscription_remove, api_user_subscriptions};
use crate::api::handler::users::apply::api_users_apply;
use crate::api::handler::users::follower::{api_users_followed, api_users_following};
use crate::api::handler::users::info::api_users_info;
use crate::api::handler::users::login::{api_users_login_email, api_users_login_name};
use crate::api::handler::users::logout::api_users_logout;
use crate::api::handler::users::repos::api_users_repos;
use crate::api::handler::users::reset::{api_user_reset_passwd_forget, api_user_reset_passwd_profile};

pub fn routes(cfg: &mut web::ServiceConfig){
    cfg
        .service(Redoc::with_url("/scalar", ApiDoc::openapi()))
        .service(
            web::scope("/user")
                .service(
                    web::scope("/applications")
                        .route("/oauth2",web::get().to(||async { "TODO" }))
                        .route("/oauth2",web::post().to(||async { "TODO" }))
                        .route("/oauth2/{id}",web::delete().to(||async { "TODO" }))
                        .route("/oauth2/{id}",web::get().to(||async { "TODO" }))
                        .route("/oauth2/{id}",web::patch().to(||async { "TODO" }))
                )
                .route("/avatar", web::post().to(api_user_avatar_upload))
                .route("/avatar", web::delete().to(api_user_avatar_delete))
                .route("/avatar", web::get().to(api_user_avatar))
                .route("/emails", web::get().to(api_user_email))
                .route("/emails", web::post().to(api_user_email_bind))
                .route("/emails", web::delete().to(api_user_email_unbind))
                .route("/followers", web::get().to(api_user_followed))
                .route("/following", web::get().to(api_user_follower))
                .route("/following/{username}", web::delete().to(api_user_unfollow))
                .route("/following/{username}", web::put().to(api_user_follow))
                .route("/gpg_keys", web::get().to(||async { "TODO" }))
                .route("/gpg_keys", web::post().to(||async { "TODO" }))
                .route("/gpg_keys/{id}", web::delete().to(||async {"TODO" }))
                .route("/gpg_keys/{id}", web::get().to(||async { "TODO" }))
                .route("/hooks", web::get().to(||async { "TODO" }))
                .route("/hooks", web::post().to(||async { "TODO" }))
                .route("/hooks/{id}", web::delete().to(||async { "TODO" }))
                .route("/hooks/{id}", web::get().to(||async { "TODO" }))
                .route("/hooks/{id}", web::patch().to(||async { "TODO" }))
                .route("/keys", web::get().to(api_user_keys))
                .route("/keys", web::post().to(api_user_key_create))
                .route("/keys/{id}", web::delete().to(api_user_key_remove))
                .route("/keys/{id}", web::get().to(api_use_key_once))
                .route("/repos", web::get().to(api_user_repo))
                .route("/repos", web::post().to(api_user_repo_create))
                .route("/settings", web::get().to(api_user_setting_get))
                .route("/settings", web::patch().to(api_user_setting_patch))
                .route("/starred", web::get().to(api_user_staring))
                .route("/starred/{owner}/{repo}", web::delete().to(api_user_star_remove))
                .route("/starred/{owner}/{repo}", web::put().to(api_user_star_add))
                .route("/subscriptions", web::get().to(api_user_subscriptions))
                .route("/subscriptions/{owner}/{repo}", web::delete().to(api_user_subscription_remove))
                .route("/subscriptions/{owner}/{repo}", web::put().to(api_user_subscription_add))
                .route("/groups", web::get().to(api_user_groups))
                .route("/times", web::patch().to(||async { "TODO"}))
        )
        .service(
            web::scope("/users")
                .service(
                    web::scope("/login")
                        .route("/email", web::post().to(api_users_login_email))
                        .route("/username", web::post().to(api_users_login_name))
                )
                .route("/logout", web::post().to(api_users_logout))
                .service(
                    web::scope("/reset")
                        .route("/forget", web::post().to(api_user_reset_passwd_forget))
                        .route("/profile", web::post().to(api_user_reset_passwd_profile))
                )
                .route("/apply", web::post().to(api_users_apply))
                .route("/search", web::get().to(||async { "TODO" }))
                .service(
                    web::scope("/once/{username}")
                        .route("/", web::get().to(api_users_info))
                        .route("/followers", web::get().to(api_users_followed))
                        .route("/following", web::get().to(api_users_following))
                        .route("/repos", web::get().to(api_users_repos))
                        .route("/starred", web::get().to(||async { "TODO" }))
                        .route("/subscriptions", web::get().to(||async { "TODO" }))
                        .route("/groups", web::get().to(||async { "TODO" }))
                        .route("/groups/{group_name}/permissions", web::get().to(||async { "TODO" }))
                )
        )
        .service(
            web::scope("/groups")
                .route("/", web::post().to(api_groups_create))
                .route("/search", web::get().to(api_groups_search))
                .service(
                    web::scope("/{group_name}")
                        .route("/", web::get().to(api_groups_info))
                        .route("/", web::post().to(||async { "TODO" }))
                        .route("/", web::delete().to(||async { "TODO" }))
                        .route("/avatar", web::get().to(api_groups_avatar))
                        .route("/avatar", web::post().to(api_groups_avatar_upload))
                        .route("/members", web::get().to(api_groups_members))
                        .route("/members/{username}", web::delete().to(api_groups_member_remove))
                        .route("/members/{username}", web::put().to(api_groups_member_add))
                        .route("/repos", web::get().to(api_groups_repo))
                        .route("/repos", web::post().to(api_groups_repo_create))
                        .route("/labels", web::get().to(api_groups_labels))
                        .route("/labels", web::post().to(api_groups_labels_create))
                        .route("/labels/{id}", web::delete().to(api_groups_labels_delete))
                        .route("/labels/{id}", web::get().to(||async { "TODO" }))
                        .route("/labels/{id}", web::patch().to(api_groups_labels_update))
                )
        )
        .service(
            web::scope("/note")
                .route("/", web::get().to(||async { "TODO" }))
                .route("/", web::post().to(||async { "TODO" }))
                .route("/{id}", web::delete().to(||async { "TODO" }))
                .route("/{id}", web::get().to(||async { "TODO" }))
                .route("/{id}", web::patch().to(||async { "TODO" }))
        )
        .service(
            web::scope("/repos")
                .route("/search", web::get().to(||async { "TODO" }))
                .service(
                    web::scope("/{owner}/{repo}")
                        .route("/", web::get().to(||async { "TODO" }))
                        .route("/", web::post().to(||async { "TODO" }))
                        .route("/", web::delete().to(||async { "TODO" }))
                        .route("/", web::patch().to(||async { "TODO" }))
                        .service(
                            web::scope("/avatar")
                                .route("/", web::get().to(||async { "TODO" }))
                                .route("/", web::put().to(||async { "TODO" }))
                        )
                        .service(
                            web::scope("/branchs")
                                .route("/", web::get().to(||async { "TODO" }))
                                .route("/", web::post().to(||async { "TODO" }))
                                .service(
                                    web::scope("/{branch}")
                                        .route("/", web::get().to(||async { "TODO" }))
                                        .route("/", web::post().to(||async { "TODO" }))
                                )
                        )
                        .service(
                            web::scope("/commits")
                                .route("/", web::get().to(||async { "TODO" }))
                                .route("/{ref}/status", web::get().to(||async { "TODO" }))
                                .route("/{sha}/pull", web::get().to(||async { "TODO" }))
                        )
                        .service(
                            web::scope("/object/{filepath}")
                                .route("/", web::get().to(||async { "TODO" }))
                                .route("/", web::post().to(||async { "TODO" }))
                                .route("/", web::put().to(||async { "TODO" }))
                                .route("/", web::delete().to(||async { "TODO" }))
                        )
                        .service(
                            web::scope("/fork")
                                .route("/", web::get().to(||async { "TODO" }))
                                .route("/", web::post().to(||async { "TODO" }))
                        )
                        .service(
                            web::scope("/pulls")
                                .route("/", web::get().to(||async { "TODO" }))
                                .route("/", web::post().to(||async { "TODO" }))
                                .service(
                                    web::scope("/{id}")
                                        .route("/", web::get().to(||async { "TODO" }))
                                        .route("/", web::post().to(||async { "TODO" }))
                                        .route("/", web::patch().to(||async { "TODO" }))
                                        .route("/", web::delete().to(||async { "TODO" }))
                                        .route("/comments", web::get().to(||async { "TODO" }))
                                        .route("/comments", web::post().to(|| async { "TODO" }))
                                        .route("/merge", web::post().to(||async { "TODO" }))
                                        .route("/files", web::get().to(||async { "TODO" }))
                                        .route("/commits", web::get().to(||async { "TODO" }))
                                        .service(
                                            web::scope("/review")
                                                .route("/", web::get().to(||async { "TODO" }))
                                                .route("/", web::post().to(||async { "TODO" }))
                                                .route("/{id}", web::delete().to(||async { "TODO" }))
                                                .route("/{id}", web::get().to(||async { "TODO" }))
                                                .route("/{id}", web::patch().to(||async { "TODO" }))
                                                .route("/comments", web::get().to(||async { "TODO" }))
                                                .route("/comments", web::post().to(||async { "TODO" }))
                                                .route("/dismissals", web::post().to(||async { "TODO" }))
                                                .route("/undismissals", web::post().to(||async { "TODO" }))
                                        )
                                        .route("/update", web::post().to(||async { "TODO" }))
                                )
                        )
                        .service(
                            web::scope("/labels")
                                .route("/", web::get().to(||async { "TODO" }))
                                .route("/", web::post().to(||async { "TODO" }))
                                .route("/{id}", web::delete().to(||async { "TODO" }))
                                .route("/{id}", web::get().to(||async { "TODO" }))
                                .route("/{id}", web::patch().to(||async { "TODO" }))
                        )
                        .service(
                            web::scope("/releases")
                                .route("/", web::get().to(||async { "TODO" }))
                                .route("/", web::post().to(||async { "TODO" }))
                                .route("/latest", web::get().to(||async { "TODO" }))
                                .service(
                                    web::scope("/{id}")
                                        .route("/", web::get().to(||async { "TODO" }))
                                        .route("/", web::post().to(||async { "TODO" }))
                                        .route("/", web::patch().to(||async { "TODO" }))
                                        .route("/", web::delete().to(||async { "TODO" }))
                                        .route("/assets", web::get().to(||async { "TODO" }))
                                        .route("/assets", web::post().to(||async { "TODO" }))
                                )
                        )
                )
        )
        .service(
            web::scope("/email")
        )
    ;
}