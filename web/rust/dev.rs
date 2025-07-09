use actix_files::NamedFile;
use actix_web::{get, HttpRequest, HttpResponse, Responder};
use actix_web::web::{scope, Path};

pub fn route(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(
            scope("/auth")
                .service(auth_page)
        )
    ;
}

#[get("/{path:.*}")]
async fn auth_page(path: Path<String>) -> impl Responder {
    if path.contains(".js") || path.contains(".css") {
        NamedFile::open(format!("web/dist/auth/{}", path.into_inner()))
    } else {
        NamedFile::open("web/dist/auth/index.html")
    }
}