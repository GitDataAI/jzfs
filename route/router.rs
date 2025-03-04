use crate::route::api::api_router;
use crate::route::git::git_router;
use actix_web::web::{scope, ServiceConfig};

pub fn router(cfg: &mut ServiceConfig) {
    cfg
        .service(
            scope("/api/v1")
                .configure(api_router)
        )
        .service(
            scope("/git")
                .configure(git_router)
        )
    ;
}