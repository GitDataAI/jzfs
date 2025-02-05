pub mod server;
pub mod api;


use actix_session::config::PersistentSession;
use actix_web::{web, App, HttpServer};
use actix_web::cookie::Key;
use actix_web::cookie::time::Duration;
use lib_config::AppNacos;
use lib_config::config::redis::RedisConfigKind;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().init();
    let nacos = AppNacos::from_env()?;
    let redis = nacos.config.redis_cluster(RedisConfigKind::Session).await?;
    let mut naming = nacos.naming.clone();
    let state = server::AppUserState::init(nacos).await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "init error"))?;
    naming
        .register(8080, "api", 1).await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "register error"))?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            // .wrap(actix_web::middleware::Logger::default())
            .wrap(
                actix_session::SessionMiddleware::builder(
                    redis.clone(),
                    Key::from(&[0; 64])
                )
                    .cookie_name("SessionID".to_string())
                    .session_lifecycle( PersistentSession::default().session_ttl(Duration::days(30)))
                    .cookie_path("/".to_string())
                    .build()
            )
            .configure(api::router)
    })
        .max_connections(usize::MAX)
        .bind("0.0.0.0:8080")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "bind error"))?
        .run()
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "run error"))?;
    naming.unregister().await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "unregister error"))?;
    Ok(())
}
