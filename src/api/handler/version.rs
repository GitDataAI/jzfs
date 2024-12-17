use std::env::consts::OS;
use actix_web::Responder;

pub async fn api_version()

-> impl Responder

{
    format!("Version: {}\nService:{}\nPid:{}\n", env!("CARGO_PKG_VERSION"),OS.to_string(),std::process::id())
}