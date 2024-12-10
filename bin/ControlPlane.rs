use std::sync::Arc;
use actix_session::config::{CookieContentSecurity, PersistentSession, TtlExtensionPolicy};
use actix_session::SessionMiddleware;
use actix_session::storage::RedisSessionStore;
use actix_web::{web, App, HttpServer};
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web_prom::PrometheusMetricsBuilder;
use log::info;
use prometheus::{opts, IntCounterVec};
use russh::{MethodSet, Preferred};
use russh::server::Server;
use russh_keys::PrivateKey;
use russh_keys::ssh_key::private::Ed25519Keypair;
use time::Duration;
use jzfs::api::routes::routes;
use jzfs::api::service::Service;
use jzfs::config::file::{Config, CFG};
use jzfs::server::db::init_db;
use jzfs::server::redis::Redis;
use jzfs::ssh::server::RusshServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    info!("Config Loading");
    Config::init().await;
    info!("Config Loaded success!");
    
    info!("Server starting");
    init_db().await;
    info!("Database init");
    let server = Service::new().await;
    info!("Service init");
    let redis = Redis::init().await;
    info!("Redis init");
    let session = RedisSessionStore::builder_pooled(redis.clone().pool).build().await?;
    info!("Session init");
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/api/metrics")
        .build()
        .unwrap();

    let counter_opts = opts!("counter", "some random counter").namespace("api");
    let counter = IntCounterVec::new(counter_opts, &["endpoint", "method", "status"]).unwrap();
    prometheus
        .registry
        .register(Box::new(counter.clone()))?;
    info!("Prometheus init");
    
    let cfg = CFG.get().unwrap().clone().http.clone();
    tokio::spawn(async {
        if std::fs::read_dir("./config").is_err() || std::fs::read("./config/id_ed25519").is_err(){
            std::fs::create_dir("./config").ok();
            let ed = Ed25519Keypair::random(&mut rand::rngs::OsRng);
            std::fs::write("./config/id_ed25519", ed.to_bytes()).unwrap();
        }
        let ed = Ed25519Keypair::from_bytes(<&[u8; 64]>::try_from(std::fs::read("./config/id_ed25519").unwrap().as_slice()).unwrap());
        if ed.is_err(){
            std::fs::remove_file("./config/id_ed25519").ok();
            let ed = Ed25519Keypair::random(&mut rand::rngs::OsRng);
            std::fs::write("./config/id_ed25519", ed.to_bytes()).unwrap();
        }
        let ed = Ed25519Keypair::from_bytes(<&[u8; 64]>::try_from(std::fs::read("./config/id_ed25519").unwrap().as_slice()).unwrap()).unwrap();

        info!("Git Server Start !!!");
        let config = russh::server::Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
            auth_rejection_time: std::time::Duration::from_secs(3),
            auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
            max_auth_attempts: 3,
            methods: MethodSet::all(),
            keys: vec![PrivateKey::from(ed)],
            preferred: Preferred {
                ..Preferred::default()
            },
            ..Default::default()
        };
        let config = Arc::new(config);
        let mut sh = RusshServer {
            service: Service::new().await,
        };
        sh.run_on_address(config, ("0.0.0.0", 2222)).await.unwrap();
    });
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
        .bind(cfg.format())?
        .workers(cfg.worker())
        .max_connections(usize::MAX)
        .run()
        .await?;
    
    Ok(())
}