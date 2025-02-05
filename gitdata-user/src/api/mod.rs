use actix_web::web;
use actix_web::web::scope;

pub mod email;
pub mod optional;
pub mod follow;
pub fn router(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            scope("/user")
                .service(
                    scope("/email")
                        .route("/can", web::get().to(email::main_email_can))
                        .route("/cannot", web::get().to(email::main_email_cannot))
                        .route("/is", web::get().to(email::main_email_is))
                        .route("/update", web::post().to(email::main_email_update))
                        .route("/add", web::post().to(email::main_email_add))
                        .route("/delete", web::post().to(email::main_email_delete))
                        .route("/list", web::get().to(email::main_email_list))
                        .route("/set_primary", web::post().to(email::main_email_set_primary))
                        .route("/set_no_primary", web::post().to(email::main_email_set_no_primary))
                )
                .service(
                    scope("/optional")
                        .route("/update", web::post().to(optional::update_optional))
                        .route("/acquire", web::get().to(optional::acquire_optional))
                )
                .service(
                    scope("/follow")
                        .route("/add", web::post().to(follow::users_follow))
                        .route("/list", web::get().to(follow::users_follow_list))
                        .route("/delete", web::post().to(follow::users_unfollow))
                )
        )
    ;
}