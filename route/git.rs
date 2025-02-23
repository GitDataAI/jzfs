use actix_web::{web, HttpResponse, Responder};
use actix_web::http::StatusCode;
use actix_web::web::{get, post};
use crate::http::{pack, refs};

pub fn git_router(cfg:&mut web::ServiceConfig) {
    cfg
        .route("/{owner}/{repo}/git-upload-pack", post().to(pack::pack))
        .route("/{owner}/{repo}/git-receive-pack", post().to(pack::pack))
        .route("/{owner}/{repo}/info/refs", get().to(refs::refs))
        .route("/{owner}/{repo}/HEAD", get().to(todo))
        .route("/{owner}/{repo}/objects/info/alternates", get().to(todo))
        .route("/{owner}/{repo}/objects/info/http-alternates", get().to(todo))
        .route("/{owner}/{repo}/objects/info/packs", get().to(todo))
        .route("/{owner}/{repo}/objects/info/{file:[^/]*}", get().to(todo))
        .route("/{owner}/{repo}/objects/{head:[0-9a-f]{2}}/{hash:[0-9a-f]{38}}", get().to(todo))
        .route("/{owner}/{repo}/objects/pack/pack-{file:[0-9a-f]{40}}.pack", get().to(todo))
        .route("/{owner}/{repo}/objects/pack/pack-{file:[0-9a-f]{40}}.idx", get().to(todo))
    ;
}

async fn todo() -> impl Responder {
    HttpResponse::build(StatusCode::PROCESSING)
        .body("Seems like an asteroid destroyed the ancient git protocol")
}