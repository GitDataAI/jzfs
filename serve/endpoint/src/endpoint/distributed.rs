use actix_web::App;
use actix_web::cookie::Key;
use actix_web::cookie::time::Duration;
use actix_web::web::Data;
use sea_orm::Iden;
use tarpc::client;
use tarpc::client::Config;
use tarpc::tokio_serde::formats::Json;
use cert::rpc::interface::CertInterFaceClient;
use session::Runtime;
use session::storage::RedisStorage;
use web_session::config::{PersistentSession, SessionLifecycle, TtlExtensionPolicy};
use web_session::middleware::SessionMiddleware;
use workhorse::rpc::proto::WorkHorseInterFaceClient;
use crate::endpoint::Endpoint;

pub async fn run() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let cert_url = std::env::var("JZFS_CERT_URL").unwrap_or("127.0.0.1:11201".to_string());
    let mut cert_transport = tarpc::serde_transport::tcp::connect(cert_url, Json::default);
    cert_transport.config_mut().max_frame_length(usize::MAX);
    let cert_client = CertInterFaceClient::new(Config::default(), cert_transport.await?).spawn();
    let workhorse_url = std::env::var("JZFS_WORKHORSE_RPC_URL").unwrap_or("127.0.0.1:11205".to_string());
    let mut workhorse_transport = tarpc::serde_transport::tcp::connect(workhorse_url, Json::default);
    workhorse_transport.config_mut().max_frame_length(usize::MAX);
    let workhorse_client = WorkHorseInterFaceClient::new(Config::default(), workhorse_transport.await?).spawn();
    let endpoint = Endpoint {
        cert: cert_client,
        workhorse: workhorse_client ,
    };
    let redis_url = std::env::var("JZFS_CACHE_URL").unwrap_or("redis://127.0.0.1:6379".to_string());
    let redis = RedisStorage::Signal(session::Config::from_url(redis_url)
        .create_pool(Some(Runtime::Tokio1))?);
    actix_web::HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(
                SessionMiddleware::builder(redis.clone(), Key::from([0; 64].as_slice()))
                    .session_lifecycle(SessionLifecycle::PersistentSession(PersistentSession {
                        session_ttl: Duration::hours(12),
                        ttl_extension_policy: TtlExtensionPolicy::OnEveryRequest,
                    }))
                    .cookie_name("session_id".into())
                    .cookie_path("/".into())
                    .build()
            )
            .app_data(Data::new(endpoint.clone()))
            .configure(web::route)
            .configure(crate::routes::run)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}