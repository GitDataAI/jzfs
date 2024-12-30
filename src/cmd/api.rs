use actix_session::config::{CookieContentSecurity, PersistentSession, TtlExtensionPolicy};
use actix_session::SessionMiddleware;
use actix_session::storage::RedisSessionStore;
use actix_web::{App, HttpServer};
use actix_web::cookie::Key;
use actix_web::cookie::time::Duration;
use actix_web::web::Data;
use tracing::info;
use crate::api::app_router::AppRouter;
use crate::config::{init_config, CFG};
use crate::server::META;
use crate::utils::db::{Init, Redis};

pub async fn api(){
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    // init_tracing_subscriber();
    info!("Starting API server...");
    init_config().unwrap();
    let cfg = CFG.get().unwrap().clone();
    info!("API server will start : {:?}", cfg.http.format());
    Init().await;
    let session = RedisSessionStore::builder_pooled(Redis().await)
        .build()
        .await.unwrap();
    info!("Redis session store initialized.");
    info!("API server started.");
    HttpServer::new(move || {
        let meta = META.get().unwrap().clone();
        App::new()
            .wrap(
                SessionMiddleware::builder(session.clone(), Key::from(&[0; 64]))
                    .cookie_name("SessionID".to_string())
                    .cookie_path("/".to_string())
                    .cookie_http_only(false)
                    .cookie_content_security(CookieContentSecurity::Private)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(Duration::days(30))
                            .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest),
                    )
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(meta))
            .configure(AppRouter)
    })
        .bind(cfg.http.starter()).unwrap()
        .run()
        .await.unwrap();
}