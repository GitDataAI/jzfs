use actix_web::cookie::Key;
use actix_web::cookie::time::Duration;
use actix_web::{App, HttpResponse};
use web_session::builder::WebSession;
use web_session::config::{PersistentSession, SessionLifecycle, TtlExtensionPolicy};
use web_session::middleware::SessionMiddleware;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let redis = session::Config::from_url("redis://172.29.124.98:6379")
        .create_pool(Some(session::Runtime::Tokio1))
        .unwrap();
    let storage = session::storage::RedisStorage::Signal(redis);
    actix_web::HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(storage.clone(), Key::from([0; 64].as_slice()))
                    .session_lifecycle(SessionLifecycle::PersistentSession(PersistentSession {
                        session_ttl: Duration::hours(12),
                        ttl_extension_policy: TtlExtensionPolicy::OnEveryRequest,
                    }))
                    .cookie_name("session_id".into())
                    .cookie_path("/".into())
                    .build(),
            )
            .service(index)
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
}

#[actix_web::get("/")]
async fn index(session: WebSession) -> HttpResponse {
    if session.0.get::<i32>("views").is_err() {
        session.0.set("views", 0);
    }
    let count = session.0.get::<i32>("views").unwrap() + 1;
    session.0.set("views", count);
    HttpResponse::Ok().body(format!("Views: {}", count))
}
