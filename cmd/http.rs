use std::{env, io};
use std::pin::{pin, Pin};
use std::task::{Context, Poll};
use actix_session::config::{BrowserSession, TtlExtensionPolicy};
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::cookie::time::Duration;
use actix_web::{
    cookie::{Key, SameSite},
    web, App, HttpServer, Responder,
};
use tracing::info;
use crate::route::router;
use crate::services::AppStateHandle;

lazy_static::lazy_static! {
    pub static ref PORT: u16 = env::var("PORT")
        .expect("PORT must setting")
        .parse()
        .expect("PORT must be number");
}

pub struct HTTPHandle;

impl HTTPHandle {
    pub async fn run_http(&self) -> std::io::Result<()> {
        info!("Http Starting...");
        let state = AppStateHandle::get().await;
        let redis_store =RedisSessionStore::builder(init_redis_store().await).build().await.map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create RedisSessionStore"))?;
        HttpServer::new(move || {
            App::new()
                .wrap(actix_web::middleware::Compress::default())
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
        })
            .bind(format!("0.0.0.0:{}", *PORT))?
            .run()
            .await
    }
}

async fn init_redis_store() -> String {
    env::var("REDIS_URL").expect("REDIS_URL must be set")
}

fn generate_secret_key() -> Key {
    Key::from(&[0; 64]) 
}

async fn index() -> impl Responder {
    "Hello, GitDataOS!"
}


impl Future for HTTPHandle {
    type Output = std::io::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        pin!(self.run_http()).poll(cx)
    }
}