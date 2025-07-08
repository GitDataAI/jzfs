use futures_util::stream::StreamExt;
use std::future;
use tarpc::client::Config;
use tarpc::server;
use tarpc::server::Channel;
use tarpc::server::incoming::Incoming;
use cert::rpc::interface::CertInterFaceClient;
use tarpc::tokio_serde::formats::Json;
use tracing::Level;
use session::Runtime;
use workhorse::rpc::proto::WorkHorseInterFace;
use workhorse::service::AppWorkHorse;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    dotenv::dotenv().ok();
    let rpc_port = std::env::var("JZFS_WORKHORSE_RPC_PORT").unwrap_or("11205".to_string()).parse::<u16>()?;
    let rpc_listen_addr = format!("0.0.0.0:{}", rpc_port);
    let mut listener = tarpc::serde_transport::tcp::listen(&rpc_listen_addr, Json::default).await?;
    tracing::info!("Listening on port {}", listener.local_addr().port());
    listener.config_mut().max_frame_length(usize::MAX);
    let db_url = std::env::var("JZFS_DATABASE_URL").expect("DATABASE_URL must be set");
    let db = sea_orm::Database::connect(db_url).await?;
    tracing::info!("Connected to database");
    let cache_url = std::env::var("JZFS_CACHE_URL").expect("CACHE_URL must be set");
    let cache = session::storage::RedisStorage::new_signal(
        deadpool_redis::Config::from_url(cache_url)
            .create_pool(Some(Runtime::Tokio1))?
    );
    tracing::info!("Connected to cache");
    let nats_url = std::env::var("JZFS_NATS_URL").expect("NATS_URL must be set");
    let nats = async_nats::connect(nats_url).await?;
    tracing::info!("Connected to NATS");
    let cert_url = std::env::var("JZFS_CERT_URL").unwrap_or("http://127.0.0.1:11201".to_string());
    let mut cert_transport = tarpc::serde_transport::tcp::connect(cert_url, Json::default);
    cert_transport.config_mut().max_frame_length(usize::MAX);
    let cert_client = CertInterFaceClient::new(Config::default(), cert_transport.await?).spawn();
    tracing::info!("Connected to cert");
    listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(u32::MAX, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = AppWorkHorse {
                db: db.clone(),
                cache: cache.clone(),
                mq: nats.clone(),
                cert: cert_client.clone(),
            };
            channel.execute(server.serve()).for_each(spawn)
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;
    Ok(())
}

async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}