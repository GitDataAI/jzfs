use libs::log::init_tracing_subscriber;
use libs::server::Init;
use russh::server::Server;
use russh::Preferred;
use std::sync::Arc;
use libs::config::init_config;
use libs::init_repo_dir;
use libs::metadata::service::MetaService;
use libs::ssh::init_git_ssh_backend;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing_subscriber();
    init_config().unwrap();
    Init().await;
    tracing::info!("Starting ssh server");
    init_repo_dir()?;
    let config = russh::server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(3),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![
            init_git_ssh_backend()
        ],
        preferred: Preferred {
            ..Preferred::default()
        },
        ..Default::default()
    };
    let mut server = libs::ssh::server::SshServer{
        server: MetaService::init().await
    };
    server.run_on_address(Arc::from(config), "0.0.0.0:2222").await?;
    Ok(())
}