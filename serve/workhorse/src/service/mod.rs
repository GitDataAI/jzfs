use chrono::NaiveDateTime;
use cert::rpc::interface::CertInterFaceClient;
use sea_orm::DatabaseConnection;
use tarpc::context::Context;
use cert::schema::AppResult;
use session::storage::RedisStorage;
use crate::rpc::proto::WorkHorseInterFace;
use crate::schema::users::UserCheckParam;

#[derive(Clone)]
pub struct AppWorkHorse {
    pub db: DatabaseConnection,
    pub cache: RedisStorage,
    pub mq: async_nats::client::Client,
    pub cert: CertInterFaceClient,
}

pub mod users;
pub mod repos;


impl WorkHorseInterFace for AppWorkHorse {
    async fn say_hello(self, _: Context) -> () {

    }

    async fn check_health(self, _: Context) -> NaiveDateTime {
        chrono::Utc::now()
            .naive_utc()
    }

    async fn user_check(self, _: Context, param: UserCheckParam) -> AppResult<i32> {
        self.service_user_check(param).await
    }
}