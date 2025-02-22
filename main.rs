#![feature(duration_constructors)]
use std::{env, io};

use actix_session::config::{BrowserSession, TtlExtensionPolicy};
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::cookie::time::Duration;
use actix_web::{
    cookie::{Key, SameSite},
    web, App, HttpServer, Responder,
};
use gitdata::app::services::AppState;
use gitdata::router::router;

lazy_static::lazy_static! {
    pub static ref PORT: u16 = std::env::var("PORT")
        .expect("PORT must setting")
        .parse()
        .expect("PORT must be number");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().init();
    
    let state = AppState::init_env().await?;
    let redis_store =RedisSessionStore::builder(init_redis_store().await).build().await.map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create RedisSessionStore"))?;

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(state.clone()))
            .wrap(
                SessionMiddleware::builder(
                    redis_store.clone(),
                    generate_secret_key(),
                )
                .cookie_name("SessionID".to_owned())
                .cookie_secure(true)
                .cookie_http_only(true)
                .cookie_same_site(SameSite::Lax)
                    .session_lifecycle(
                        BrowserSession::default()
                            .state_ttl(Duration::days(30))
                            .state_ttl_extension_policy(
                                TtlExtensionPolicy::OnEveryRequest,
                            )
                    )
                .build(),
            )
            .service(web::resource("/").to(index))
            .configure(router)
            // 在此添加其他路由
    })
    .bind(format!("0.0.0.0:{}", *PORT))?
    .run()
    .await
}

async fn init_redis_store() -> String {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    redis_url
}

fn generate_secret_key() -> Key {
    Key::from(&[0; 64]) // 生产环境应使用真实密钥
}

async fn index() -> impl Responder {
    "Hello, GitDataOS!"
}
