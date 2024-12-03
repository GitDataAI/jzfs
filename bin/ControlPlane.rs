use std::collections::HashMap;
use std::thread;
use actix_session::config::{CookieContentSecurity, PersistentSession, TtlExtensionPolicy};
use actix_session::SessionMiddleware;
use actix_session::storage::RedisSessionStore;
use actix_web::{web, App, HttpServer};
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::rt::System;
use actix_web_prom::PrometheusMetricsBuilder;
use log::info;
use prometheus::{opts, IntCounterVec};
use time::Duration;
use jzfs::api::routes::routes;
use jzfs::api::service::Service;
use jzfs::server::db::init_db;
use jzfs::server::redis::Redis;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    info!("server start");
    init_db().await;
    info!("db init");
    let server = Service::new().await;
    info!("service init");
    let redis = Redis::init().await;
    info!("redis init");
    let session = RedisSessionStore::builder_pooled(redis.clone().pool).build().await?;
    info!("session init");
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/api/metrics")
        .build()
        .unwrap();

    let counter_opts = opts!("counter", "some random counter").namespace("api");
    let counter = IntCounterVec::new(counter_opts, &["endpoint", "method", "status"]).unwrap();
    prometheus
        .registry
        .register(Box::new(counter.clone()))?;
    info!("prometheus init");
    HttpServer::new(move || { 
        App::new()
            .wrap(Logger::default())
            .wrap(prometheus.clone())
            .app_data(web::Data::new(counter.clone()))
            .wrap(
                SessionMiddleware::builder(
                    session.clone(),
                    Key::from(&[0; 64])
                )
                    .cookie_name("SessionID".to_string())
                    .cookie_path("/".to_string())
                    .cookie_http_only(false)
                    .cookie_content_security(
                        CookieContentSecurity::Private
                    )
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(Duration::days(30))
                            .session_ttl_extension_policy(TtlExtensionPolicy::OnStateChanges)
                    )
                    .cookie_secure(false)
                    .build()
            )
            .app_data(web::Data::new(server.clone()))
            .service(
                web::scope("/api")
                    .configure(routes)
            )
    })
        .bind("0.0.0.0:80")?
        .max_connections(usize::MAX)
        .run()
        .await?;
    
    Ok(())
}