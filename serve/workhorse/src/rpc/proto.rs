use chrono::NaiveDateTime;
use tarpc::service;

#[service]
pub trait WorkHorseInterFace {
    async fn say_hello();
    async fn check_health() -> NaiveDateTime;
}
