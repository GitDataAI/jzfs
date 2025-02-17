use poem::{get, patch, post, Route};
use crate::app::api::handler::auth::{auth_apply, auth_captcha, auth_check, auth_email_check, auth_email_send, auth_logout, auth_passwd};
use crate::app::api::handler::repo::{repo_bhct, repo_create, repo_info, repo_tree};
use crate::app::api::handler::static_file::{down_avatar, upload_avatar};
use crate::app::api::handler::users::{user_dashbored, user_now, user_update_optional};
use crate::app::http::git_router;

pub fn router() -> Route {
    Route::new()
        .nest(
            "/api",
            Route::new()
                .nest(
                    "/auth",
                    Route::new()
                        .at("/passwd", post(auth_passwd))
                        .at("/apply", post(auth_apply))
                        .at("/logout", post(auth_logout))
                        .at("/captcha", get(auth_captcha))
                        .at("/email_send", post(auth_email_send))
                        .at("/email_check", post(auth_email_check))
                        .at("/check", post(auth_check))
                )
                .nest(
                    "/static",
                    Route::new()
                        .at("/upload_avatar", post(upload_avatar))
                        .at("/:path", post(down_avatar))
                )
                .nest(
                    "/user",
                    Route::new()
                        .at("/now",get(user_now))
                        .at("/uptional", patch(user_update_optional))
                        .at("/:username/dashbored", get(user_dashbored))
                )
                .nest(
                    "/repo",
                    Route::new()
                        .at("/create", post(repo_create))
                        .at("/:owner/:repo/bhct", post(repo_bhct))
                        .at("/:owner/:repo/info", post(repo_info))
                        .at("/:owner/:repo/branch/:branch/:head/tree", post(repo_tree))
                )
        )
       
        
        .nest("/git", git_router())
}