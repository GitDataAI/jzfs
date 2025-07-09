use chrono::NaiveDateTime;
use tarpc::service;
use cert::schema::AppResult;
use crate::schema::users::UserCheckParam;

#[service]
pub trait WorkHorseInterFace {
    async fn say_hello();
    async fn check_health() -> NaiveDateTime;
    async fn user_check(param: UserCheckParam) -> AppResult<i32>;
}
