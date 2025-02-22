use actix_web::web;
use actix_web::web::{get, patch, post, scope};
use crate::app::api::handler::auth::{auth_apply, auth_captcha, auth_check, auth_email_check, auth_email_send, auth_logout, auth_passwd};
use crate::app::api::handler::explore::explore_repo_hot;
use crate::app::api::handler::repo::{repo_access, repo_bhct, repo_create, repo_file, repo_fork, repo_info, repo_star, repo_tree, repo_watch};
use crate::app::api::handler::static_file::{down_avatar, upload_avatar};
use crate::app::api::handler::users::{user_dashbored, user_info_by_uid, user_now, user_update_optional};
use crate::app::http::git_router;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            scope("/api")
                .service(
                    scope("/auth")
                        .route("/passwd", post().to(auth_passwd))
                        .route("/apply", post().to(auth_apply))
                        .route("/logout", post().to(auth_logout))
                        .route("/captcha", get().to(auth_captcha))
                        .route("/email_send", post().to(auth_email_send))
                        .route("/email_check", post().to(auth_email_check))
                        .route("/check", post().to(auth_check))
                )
                .service(
                    scope("/explore")
                        .route("/repo", patch().to(explore_repo_hot))
                )
                .service(
                    scope("/static")
                        .route("/upload_avatar", post().to(upload_avatar))
                        .route("/img/{path}", get().to(down_avatar))
                )
                .service(
                    scope("/user")
                        .route("/now", get().to(user_now))
                        .route("/uptional", patch().to(user_update_optional))
                        .route("/{username}/dashbored", get().to(user_dashbored))
                        .route("/uid/{uid}", post().to(user_info_by_uid))
                )
                .service(
                    scope("/repo")
                        .route("/file", post().to(repo_file))
                        .route("/create", post().to(repo_create))
                        .route("/access",get().to(repo_access))
                        .route("/{owner}/{repo}/bhct", post().to(repo_bhct))
                        .route("/{owner}/{repo}/info", post().to(repo_info))
                        .route("/{owner}/{repo}/branch/{branch}/{head}/tree", post().to(repo_tree))
                        .route("/{owner}/{repo}/fork", post().to(repo_fork))
                        .route("/{owner}/{repo}/star", post().to(repo_star))
                        .route("/{owner}/{repo}/watch/{level}", post().to(repo_watch))
                )
        )
        .service(
            scope("/git")
                .configure(git_router)
        )
    ;
}