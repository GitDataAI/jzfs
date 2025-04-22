use actix_web::{web, Responder};
use jz_module::AppModule;
use jz_module::explore::ExploreParma;

pub async fn explore(
    module: web::Data<AppModule>,
    params: web::Query<ExploreParma>,
) -> impl Responder {
    match module.explore(params.into_inner()).await {
        Ok(data) => actix_web::HttpResponse::Ok().json(data),
        Err(err) => actix_web::HttpResponse::InternalServerError().body(err.to_string()),
    }
}