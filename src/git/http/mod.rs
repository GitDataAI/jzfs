use actix_web::web;

pub mod backend;
pub mod server;
#[allow(non_snake_case)]
pub fn GitHttpBackend(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/{owner}/{repo_name}").configure(backend::routes));
}
