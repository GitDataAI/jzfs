#![feature(duration_constructors)]
#![allow(clippy::module_inception)]

use poem::session::{CookieConfig, RedisStorage, ServerSession};
use poem::{get, handler, listener::TcpListener, EndpointExt};
use redis::aio::ConnectionManager;
use redis::Client;
use std::env;
use std::time::Duration;
use tracing::info;
use gitdata::app::services::AppState;
use gitdata::router::router;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt().init();
    let listener = TcpListener::bind("0.0.0.0:80");
    let state = AppState::init_env().await?;
    let app = router()
        .at("/", get(index))
        .with(poem::middleware::Tracing)
        .data(state)
        .with(ServerSession::new(
            CookieConfig::new()
                .path("/")
                .name("SessionID")
                .max_age(Duration::from_days(31))
                .http_only(true)
                .secure(true),
            redis().await,
        ));
    poem::Server::new(listener)
        .run(app)
        .await
}




#[handler]
async fn index() -> String {
    "Hello, GitDataOS!".to_string()
}

async fn redis() -> RedisStorage<ConnectionManager> {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = Client::open(redis_url).expect("Invalid redis url");
    info!("Connected to redis");
    let con = ConnectionManager::new(client).await.expect("Failed to connect to redis");
    RedisStorage::new(con)
}