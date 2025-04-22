use actix_web::web::{get, scope};
use crate::repo::repo_list;

pub mod repo;
pub mod bare;

pub fn openapi_router(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(
            scope("/repo")
                .route("",get().to(repo_list))
        )
    ;
}