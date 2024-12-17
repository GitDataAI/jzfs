use actix_web::{web, App, HttpServer};
use actix_web::web::scope;
use tracing::info;
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
    HttpServer::new(|| {
        let meta = META.get().unwrap().clone();
        App::new()
            .app_data(web::Data::new(meta))
            .wrap(ActixServer)
            .service(
                scope("/api")
                    .route("/version", web::get().to(api_version))
                    .configure(app_routes::routes)
            )
    })
        .bind(cfg.http.starter())?
        .run()
        .await?;
    info!("API server started.");
    Ok(())
}