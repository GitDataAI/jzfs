use std::io;
use std::pin::{pin, Pin};
use std::sync::Arc;
use std::task::{Context, Poll};
use russh::keys::PrivateKey;
use russh::keys::ssh_key::private::Ed25519Keypair;
use russh::server::{Config, Server};
use russh::{MethodSet, SshId};
use tracing::info;
use crate::ssh::server::SSHServer;

pub struct SSHHandle;

impl SSHHandle {
    pub async fn run_ssh(&self) -> io::Result<()>{
        info!("SSH Starting...");
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
}

impl Future for SSHHandle {
    type Output = std::io::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        pin!(self.run_ssh()).poll(cx)
    }
}