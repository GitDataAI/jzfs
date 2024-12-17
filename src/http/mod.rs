use crate::config::init_config;
use crate::http::handler::{get_text_file, git_receive_pack, git_upload_pack, info_refs, objects_info_packs, objects_pack};
use crate::log::init_tracing_subscriber;
use crate::metadata::service::MetaService;
use crate::server::Init;
use actix_web::{web, App, HttpServer};
use crate::http::config::init_git_http_backend;
use crate::init_repo_dir;

pub mod server;
pub mod handler;
pub mod config;

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
            .service(web::scope("/{owner}/{repo_name}")
                .route("/git-upload-pack", web::to(git_upload_pack))
                .route("/git-receive-pack", web::to(git_receive_pack))
                .route("info/refs", web::to(info_refs))
                .route("HEAD", web::to(get_text_file))
                .route("objects/info/alternates", web::to(get_text_file))
                .route("objects/info/http-alternates", web::to(get_text_file))
                .route("objects/info/packs", web::to(objects_info_packs))
                .route("objects/info/{rest:.*}", web::to(get_text_file))
                .route("objects/pack/{pack}", web::to(objects_pack))
            )
    })
        .bind("0.0.0.0:80")?
        .run()
        .await?;
    Ok(())
}

