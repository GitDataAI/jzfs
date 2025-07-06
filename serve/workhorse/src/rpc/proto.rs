use tarpc::service;

#[service]
pub trait WorkHorseInterFace {
    async fn say_hello();
}