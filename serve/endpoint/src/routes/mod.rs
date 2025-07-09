mod cert;

use crate::endpoint::Endpoint;
use crate::routes::cert::{email_captcha, email_verify, users_login, users_logout, users_register};
use actix_web::web::{get, post, scope, Data, ServiceConfig};
use actix_web::{HttpResponse, Responder};
use serde_json::{json, Value};
use tarpc::context::Context;

pub fn run(cfg: &mut ServiceConfig) {
    cfg
        .configure(web::route)
        .service(
        scope("/api")
            .route("/health", get().to(health_check))
            .service(
                scope("/auth")
                    .route("/login", post().to(users_login))
                    .route("/register", post().to(users_register))
                    .route("/logout", post().to(users_logout))
                    .route("/email/captcha", post().to(email_captcha))
                    .route("/email/verify", post().to(email_verify)),
            )
        ,
    );
}



#[cfg(feature = "distributed")]
async fn health_check(data: Data<Endpoint>) -> impl Responder {
    let now = chrono::Utc::now().naive_utc();
    let context = Context::current();
    let mut json = Value::Null;
    if let Ok(cret) = data.cert.health_check(context).await {
        let ttl = cret.signed_duration_since(now).as_seconds_f64() * 1000.;
        json["CRET SERVICE"] = json!({
            "status": "OK",
            "ttl": format!("{} ms", ttl),
        })
    } else {
        json["CRET SERVICE"] = json!({
            "status": "ERROR",
        })
    }
    if let Ok(workhorse) = data.workhorse.check_health(context).await {
        let ttl = workhorse.signed_duration_since(now).as_seconds_f64() * 1000.;
        json["WORKHOUSE SERVICE"] = json!({
            "status": "OK",
            "ttl": format!("{} ms", ttl),
        })
    } else {
        json["WORKHOUSE SERVICE"] = json!({
            "status": "ERROR",
        })
    }
    json["mode"] = Value::String("distributed".to_string());
    HttpResponse::Ok().json(json)
}

#[cfg(feature = "local")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "ok", mode: "local" }))
}
