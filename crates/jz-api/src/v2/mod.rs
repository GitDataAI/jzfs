use actix_web::web;

pub mod auth;
pub mod context;

pub async fn v2_router(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/context", web::patch().to(context::list_context))
        .route("/context", web::get().to(context::current_context))
        .route("/context/{uid}", web::post().to(context::switch_context))
    ;
}