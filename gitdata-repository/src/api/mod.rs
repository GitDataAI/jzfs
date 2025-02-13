mod avatar;
mod branches;
mod create;
mod info;

use actix_web::web;
use actix_web::web::delete;
use actix_web::web::get;
use actix_web::web::post;
use actix_web::web::scope;

use crate::api::avatar::delete_avatar;
use crate::api::avatar::update_avatar;
use crate::api::create::create_repository;
use crate::api::info::repository_info;

pub fn routes(cfg : &mut web::ServiceConfig) {
    cfg.service(
        scope("/repository")
            .route("/create", post().to(create_repository))
            .service(
                scope("/{owner}/{repo}")
                    .route("", get().to(repository_info)) // repository base info
                    .route("/refs", get().to(todo)) // repository refs
                    .service(
                        scope("/branches")
                            .route("{list}", get().to(branches::list_branch))
                            .service(
                                scope("/{name}")
                                    .route("", post().to(branches::create_branch))
                                    .route("", delete().to(branches::delete_branch)),
                            ),
                    )
                    .service(scope("/commits/{branches}"))
                    .service(
                        scope("/avatar")
                            .route("/update", post().to(update_avatar))
                            .route("/delete", post().to(delete_avatar)),
                    ),
            ),
    );
}

async fn todo() -> String {
    "todo".to_string()
}
