#![feature(duration_constructors)]

use tracing::warn;
use gitdata::cmd::http::HTTPHandle;
use gitdata::cmd::ssh::SSHHandle;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().init();
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
