use sea_orm::DatabaseConnection;
use cert::rpc::interface::CertInterFaceClient;
use session::storage::RedisStorage;


#[derive(Clone)]
pub struct AppWorkHorse {
    pub db: DatabaseConnection,
    pub cache: RedisStorage,
    pub mq: async_nats::client::Client,
    pub cert: CertInterFaceClient,
}
