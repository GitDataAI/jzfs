use actix_session::config::{BrowserSession, TtlExtensionPolicy};
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::web::scope;
use actix_web::{web, App, HttpServer};
use jz_module::AppModule;

pub mod utils;
pub mod v1;
pub mod v2;
pub use actix_settings::*;
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
use jz_dragonfly::Dragonfly;
use log::info;
use lazy_static::lazy_static;
use jz_service::app::AppService;

pub struct Api {
    pub module: AppModule,
    pub config: Settings,
    pub service: AppService,
}

lazy_static!{
    pub static ref PROTS: u16 = std::env::var("PORT").unwrap_or("9000".to_string()).parse().unwrap();
}


impl Api {
    pub fn init(module: AppModule, config: Settings, service: AppService) -> Api {
        Api { module, config , service }
    }
    pub async fn run(&self) -> anyhow::Result<()> {
        use anyhow::Context;

        let session_pool = RedisSessionStore::builder_pooled(Dragonfly::connect_pool()).build().await.expect("Failed to create session pool");
        info!("Dragonfly Connect Successful!!");
        let session_key = Key::from([0;64].as_slice());
        let module = self.module.clone();
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(module.clone()))
                .wrap(actix_web::middleware::Logger::default())
                .wrap(
                    SessionMiddleware::builder(
                        session_pool.clone(),
                        session_key.clone(),
                    )
                        .cookie_path("/".to_string())
                        .cookie_name("SessionID".to_string())
                        .cookie_secure(true)
                        .session_lifecycle(
                            BrowserSession::default()
                                .state_ttl(Duration::days(30))
                                .state_ttl_extension_policy(
                                    TtlExtensionPolicy::OnEveryRequest
                                )
                        )
                        .build()
                )
                .route("/", actix_web::web::get().to(|| async { "Hello World!" }))
                .service(
                    scope("/api")
                        .service(
                            scope("/v1")
                                .configure(v1::v1_route)
                        )
                        .service(
                            scope("/v2")
                        )
                )
                .service(scope("/git").configure(jz_smart::git_router))
                .service(scope("/openapi").configure(jz_openapi::openapi_router))
        })
        .bind(format!("0.0.0.0:{}", PROTS.to_string()))
        .context("Failed to apply actix settings")?
        .run()
        .await
        .context("Failed to run actix service")?;
        Ok(())
    }
}