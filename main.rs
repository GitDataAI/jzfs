#![feature(duration_constructors)]

use tracing::warn;
use gitdata::cmd::http::HTTPHandle;
use gitdata::cmd::ssh::SSHHandle;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;


// main.rs
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;
#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
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
