use cert::rpc::interface::CertInterFaceClient;
use sea_orm::DatabaseConnection;
use session::storage::RedisStorage;

#[derive(Clone)]
pub struct AppWorkHorse {
    pub db: DatabaseConnection,
    pub cache: RedisStorage,
    pub mq: async_nats::client::Client,
    pub cert: CertInterFaceClient,
}

pub mod users;
