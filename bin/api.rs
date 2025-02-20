use russh::keys::ssh_key::private::{Ed25519Keypair};
use std::io;
use std::sync::Arc;
use russh::keys::PrivateKey;
use russh::server::{Config, Server};
use russh::{MethodSet, SshId};
use tracing::info;
use gitdata::app::ssh::server::SSHServer;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt().init();
    info!("Starting...");
    let start = std::time::Instant::now();
    let env = std::env::var("ED25519").expect("ED25519 not set");
    let vec = env.split(",")
        .map(|x|x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let key_bytes: [u8; 64] = vec.try_into().expect("Invalid key length");
    let key = Ed25519Keypair::from_bytes(&key_bytes).unwrap();
    let mut config = Config::default();
    config.keys = vec![PrivateKey::from(key)];
    let version = format!("SSH-2.0-Gitdata {}", env!("CARGO_PKG_VERSION"));
    config.server_id = SshId::Standard(version);
    config.methods = MethodSet::all();
    let mut server = SSHServer::from_env().await?;

    server.run_on_address(Arc::new(config), "0.0.0.0:2322").await?;
    
    
    println!("{:?}", start.elapsed());

    Ok(())
}