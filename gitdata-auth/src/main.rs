pub mod api;
pub mod server;

use actix_session::config::PersistentSession;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::cookie::Key;
use actix_web::cookie::time::Duration;
use actix_web::web;
use lazy_static::lazy_static;
use lib_config::AppNacos;
use lib_config::config::redis::RedisConfigKind;
use lib_mq::client::client::AppKafkaClient;

lazy_static! {
    static ref PORT: u16 = {
        let port = std::env::var("ALL_PORT").unwrap_or("8080".to_string());
        port.parse::<u16>().unwrap_or(8080)
    };
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().init();
    let nacos = AppNacos::from_env()?;
    let redis = nacos.config.redis_cluster(RedisConfigKind::Session).await?;
    let mut naming = nacos.naming.clone();
    let state = server::AppAuthState::init(nacos.clone())
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "init error"))?;
    naming
        .register(PORT.clone() as i32, "api", 1)
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "register error"))?;
    let mq = AppKafkaClient::init(nacos).await?;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(mq.clone()))
            // .wrap(actix_web::middleware::Logger::default())
            .wrap(
                actix_session::SessionMiddleware::builder(redis.clone(), Key::from(&[0; 64]))
                    .cookie_name("SessionID".to_string())
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(30)))
                    .cookie_path("/".to_string())
                    .build(),
            )
            .configure(api::router)
    })
    .max_connections(usize::MAX)
    .bind(format!("0.0.0.0:{}", PORT.clone()))
    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "bind error"))?
    .run()
    .await
    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "run error"))?;
    naming
        .unregister()
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "unregister error"))?;
    Ok(())
}
