use crate::api::handlers::emails::handlers::{email_captcha, email_captcha_check};
use crate::api::handlers::repos::blob::repo_blob_sha;
use crate::api::handlers::repos::branchs::{
    repo_branch_delete, repo_branch_info, repo_branch_list, repo_branch_protect,
    repo_branch_unprotect, repo_get_default_branch, repo_set_default_branch,
};
use crate::api::handlers::repos::commits::{repo_commit_list, repo_commit_sha};
use crate::api::handlers::repos::repos::{repo_create, repo_info, repo_search};
use crate::api::handlers::repos::star::{
    repo_star_add, repo_star_count, repo_star_list, repo_star_remove,
};
use crate::api::handlers::repos::tree::repo_tree;
use crate::api::handlers::repos::watch::{
    repo_watch_add, repo_watch_count, repo_watch_list, repo_watch_remove,
};
use crate::api::handlers::user::avatar::{avatar_get, avatar_set};
use crate::api::handlers::user::email::{users_email_add, users_email_del, users_email_get};
use crate::api::handlers::user::followers::{
    users_follower_add, users_follower_count, users_follower_del, users_follower_get,
};
use crate::api::handlers::user::following::{users_following_count, users_following_get};
use crate::api::handlers::user::ssh_key::{
    users_key_add, users_key_del, users_key_get, users_key_get_by_uid,
};
use crate::api::handlers::users::apply::apply;
use crate::api::handlers::users::handlers::{user_follower_count_data, users_info_username};
use crate::api::handlers::users::login::{login, logout, session};
use crate::api::handlers::users::search::users_search;
use crate::git::http::GitHttpBackend;
use actix_web::web;
use actix_web::web::{delete, get, patch, post, put, scope};
use crate::api::handlers::repos::check::{check_repo_name, repo_owner_check};
use crate::api::handlers::user::repo::{user_option, users_repos};

#[allow(non_snake_case)]
pub fn AppRouter(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/api")
            .service(scope("/git").configure(GitHttpBackend))
            .service(
                scope("/v1")
                    .service(
                        scope("/users")
                            .route("/login", post().to(login))
                            .route("/logout", post().to(logout))
                            .route("/apply", post().to(apply))
                            .route("/search", get().to(users_search))
                            .route("/repo",get().to(users_repos))
                            .route("/follow", get().to(user_follower_count_data))
                            .route("/information/{username}", get().to(users_info_username)),
                    )
                    .service(
                        scope("/user")
                            .route("", put().to(user_option))
                            .route("", post().to(session))
                            .service(
                                scope("/avatar")
                                    .route("", get().to(avatar_get))
                                    .route("", put().to(avatar_set)),
                            )
                            .service(
                                scope("/ssh_key")
                                    .route("", post().to(users_key_add))
                                    .route("", get().to(users_key_get))
                                    .route("/{uid}", delete().to(users_key_del))
                                    .route("/{uid}", get().to(users_key_get_by_uid)),
                            )
                            .service(
                                scope("/follower")
                                    .route("", get().to(users_follower_get))
                                    .route("/count", get().to(users_follower_count))
                                    .service(
                                        scope("/{username}")
                                            .route("", post().to(users_follower_add))
                                            .route("", delete().to(users_follower_del)),
                                    ),
                            )
                            .service(
                                scope("/following")
                                    .route("/count", get().to(users_following_count))
                                    .route("", get().to(users_following_get)),
                            )
                            .service(
                                scope("/email")
                                    .route("", get().to(users_email_get))
                                    .route("", post().to(users_email_add))
                                    .route("", delete().to(users_email_del)),
                            ),
                    )
                    .service(
                        scope("/repo")
                            .route("", post().to(repo_create))
                            .route("/{keyword}", get().to(repo_search))
                            .service(
                                scope("/check")
                                    .route("/owner", get().to(repo_owner_check))
                                    .route("/name/{owner}/{repo}", get().to(check_repo_name))
                            )
                            .service(
                                scope("/{owner}/{repos}")
                                    .route("", get().to(repo_info))
                                    .service(
                                        scope("/branch")
                                            .route("", get().to(repo_branch_list))
                                            .service(
                                                scope("/{branch}")
                                                    .route("", get().to(repo_branch_info))
                                                    .route("", delete().to(repo_branch_delete))
                                                    .route("", delete().to(repo_branch_delete))
                                                    .route("/tree", get().to(repo_tree))
                                                    .route("/tree/{sha}", get().to(repo_tree))
                                                    .route("/blob", get().to(repo_blob_sha))
                                                    .route(
                                                        "/protect",
                                                        post().to(repo_branch_protect),
                                                    )
                                                    .route(
                                                        "/unprotect",
                                                        delete().to(repo_branch_unprotect),
                                                    )
                                                    .service(
                                                        scope("/commits")
                                                            .route("", get().to(repo_commit_list))
                                                            .route(
                                                                "/{sha}",
                                                                get().to(repo_commit_sha),
                                                            )
                                                            .route(
                                                                "/{sha}/files",
                                                                get().to(repo_blob_sha),
                                                            ),
                                                    ),
                                            ),
                                    )
                                    .service(
                                        scope("/default_branch")
                                            .route("", get().to(repo_get_default_branch))
                                            .route("", put().to(repo_set_default_branch)),
                                    )
                                    .service(
                                        scope("/star")
                                            .route("", get().to(repo_star_count))
                                            .route("", patch().to(repo_star_list))
                                            .route("", post().to(repo_star_add))
                                            .route("", delete().to(repo_star_remove)),
                                    )
                                    .service(
                                        scope("/watch")
                                            .route("", get().to(repo_watch_count))
                                            .route("", patch().to(repo_watch_list))
                                            .route("/{level}", post().to(repo_watch_add))
                                            .route("", delete().to(repo_watch_remove))
                                            .route("/{level}", put().to(repo_watch_count)),
                                    ),
                            ),
                    )
                    .service(scope("/group"))
                    .service(scope("/teams"))
                    .service(scope("/issues"))
                    .service(scope("/pulls"))
                    .service(scope("/hook"))
                    .service(scope("/comments"))
                    .service(
                        scope("/email")
                            .route("/captcha", post().to(email_captcha))
                            .route("/captcha", put().to(email_captcha_check)),
                    ),
            ),
    )
    .service(web::scope("/rpc"));
}
