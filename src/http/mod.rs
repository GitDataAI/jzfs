use crate::config::init_config;
use crate::http::config::init_git_http_backend;
use crate::init_repo_dir;
use crate::log::init_tracing_subscriber;
use crate::metadata::service::MetaService;
use crate::server::Init;
use actix_web::{web, App, HttpServer};

pub mod server;
pub mod handler;
pub mod config;
pub mod backend;
#[allow(non_snake_case)]
pub async fn GitHttp() -> anyhow::Result<()>{
    init_tracing_subscriber();
    init_config().unwrap();
    init_git_http_backend()?;
    Init().await;
    init_repo_dir()?;
    let service = MetaService::init().await;
    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(service.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::scope("/{owner}/{repo_name}")
                    .configure(backend::routes)
            )
    })
        .bind("0.0.0.0:80")?
        .run()
        .await?;
    Ok(())
}

