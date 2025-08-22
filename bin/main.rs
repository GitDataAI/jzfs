use crate::welcome::welcome;
use lazy_static::lazy_static;

pub mod api;
pub mod cli;
pub mod welcome;
mod webhook;

lazy_static! {
    pub static ref DEV: bool = {
        match std::env::var("DEV") {
            Ok(s) => s == "true",
            Err(_) => false,
        }
    };
}

#[tokio::main]
async fn main() {
    welcome();
    if DEV.clone() {
        if let Err(e) = api::api().await {
            eprintln!("{}", e.msg);
            std::process::abort();
        }
    } else {
        if let Err(e) = cli::cli().await {
            eprintln!("{}", e.msg);
            std::process::abort();
        }
    }
}
