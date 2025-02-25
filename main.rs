#![feature(duration_constructors)]

use env_logger::Builder;
use tracing::{log, warn};
use gitdata::cmd::http::HTTPHandle;
use gitdata::cmd::ssh::SSHHandle;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    let http_handle = HTTPHandle{}.run_http();
    let ssh_handle = SSHHandle{}.run_ssh();
    tokio::select! {
        _ = http_handle => {
            warn!("HTTP server stopped");
        }
        _ = ssh_handle => {
            warn!("SSH server stopped");
        }
        _ = tokio::signal::ctrl_c() => {
            warn!("Ctrl+C received, shutting down...");
        }
    }
    Ok(())
}
