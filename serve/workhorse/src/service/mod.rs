use chrono::NaiveDateTime;
use cert::rpc::interface::CertInterFaceClient;
use sea_orm::DatabaseConnection;
use tarpc::context::Context;
use session::storage::RedisStorage;
use crate::rpc::proto::WorkHorseInterFace;

#[derive(Clone)]
pub struct AppWorkHorse {
    pub db: DatabaseConnection,
    pub cache: RedisStorage,
    pub mq: async_nats::client::Client,
    pub cert: CertInterFaceClient,
}

pub mod users;


impl WorkHorseInterFace for AppWorkHorse {
    async fn say_hello(self, _: Context) -> () {

    }

    async fn check_health(self, _: Context) -> NaiveDateTime {
        chrono::Utc::now()
            .naive_utc()
    }
}