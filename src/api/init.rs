use actix_session::config::{CookieContentSecurity, PersistentSession, TtlExtensionPolicy};
use actix_session::SessionMiddleware;
use actix_session::storage::RedisSessionStore;
use actix_web::{web, App, HttpServer};
use actix_web::cookie::Key;
use actix_web::web::scope;
use time::Duration;
use tracing::info;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use crate::api::app_docs::ApiDoc;
use crate::api::app_error::Error;
use crate::api::app_routes;
use crate::api::handler::version::api_version;
use crate::api::middleware::service::ActixServer;
use crate::config::{init_config, CFG};
use crate::init_repo_dir;
use crate::log::init_tracing_subscriber;
use crate::metadata::service::META;
use crate::server::Init;

pub async fn init_api() -> Result<(), Error>{
    init_tracing_subscriber();
    info!("Starting API server...");
    init_repo_dir().unwrap();
    init_config()?;
    let cfg = CFG.get().unwrap().clone();
    info!("API server will start : {:?}", cfg.http.format());
    Init().await;
    let session = RedisSessionStore::builder_pooled(META.get().unwrap().redis()).build().await.unwrap();
    info!("Redis session store initialized.");
    info!("API server started.");
    HttpServer::new(move || {
        let meta = META.get().unwrap().clone();
        App::new()
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
                            .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest)
                    )
                    .cookie_secure(false)
                    .build()
            )
            .app_data(web::Data::new(meta))
            .wrap(ActixServer)
            .service(
                scope("/api")
                    .service(Redoc::with_url("/openapi", ApiDoc::openapi()))
                    .route("/version", web::get().to(api_version))
                    .service(
                        scope("/v1")
                            .configure(app_routes::routes)
                    )
            )
    })
        .bind(cfg.http.starter())?
        .run()
        .await?;
    Ok(())
}