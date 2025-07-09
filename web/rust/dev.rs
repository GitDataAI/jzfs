use awc::Client;

use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::Bytes;
pub fn route(cfg: &mut web::ServiceConfig) {
    cfg
        .service(auth_proxy);
}


#[get("/auth/{url:.*}")]
async fn auth_proxy(
    path: web::Path<(String,)>,
    client: web::Data<Client>,
) -> impl Responder {
    let (url,) = path.into_inner();
    let url = format!("http://127.0.0.1:3001/{url}");
    if let Ok(mut res) = client.get(&url).send().await{
        let mut r =  HttpResponse::Ok();
        let body = res.body().await;
        let header = res.headers();
        for (key, value) in header.iter() {
            r.insert_header((key, value));
        }
        r.status(res.status());
        r.body(body.unwrap_or(Bytes::new()))
    }else {
        HttpResponse::InternalServerError().body("Error")
    }
}