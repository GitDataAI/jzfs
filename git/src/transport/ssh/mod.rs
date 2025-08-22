use crate::service::GitServer;
use crate::transport::ssh::server::SSHServer;
use russh::keys::PrivateKey;
use russh::keys::ssh_key::private::Ed25519Keypair;
use russh::server::{Config, Server};
use russh::{MethodSet, SshId};
use std::sync::Arc;
use tracing::info;

pub mod handle;
pub mod server;

#[derive(Clone)]
pub struct SSHHandle {
    pub app: GitServer,
}

impl SSHHandle {
    pub async fn run(&self) {
        let this = self.clone();
        tokio::spawn(async move {
            this.run_ssh().await.ok();
        });
    }
    pub fn new(app: GitServer) -> Self {
        Self { app }
    }
    pub async fn run_ssh(&self) -> anyhow::Result<()> {
        info!("SSH Starting...");
        let ed25519 = self.app.config.ssh.ed25519_hex.clone();
        let vec = hex::decode(ed25519.as_bytes().to_vec())?;
        let key_bytes: [u8; 64] = vec.try_into().expect("Invalid key length");
        let key = Ed25519Keypair::from_bytes(&key_bytes)?;
        let mut config = Config::default();
        config.keys = vec![PrivateKey::from(key)];
        let version = format!("SSH-2.0-Gitdata {}", env!("CARGO_PKG_VERSION"));
        config.server_id = SshId::Standard(version);
        config.methods = MethodSet::all();
        config.maximum_packet_size = 65535;
        let mut server = SSHServer::new(self.app.clone());
        let addr = format!("{}:{}", self.app.config.ssh.host, self.app.config.ssh.port);
        if self.app.config.ssh.port == 22 {
            info!(
                "Ssh server run on:{}, you can use git cli `git clone git@{}:{{owner}}/{{name}}`",
                addr.clone(),
                addr.clone()
            )
        } else {
            info!(
                "Ssh server run on:{}, you can use git cli `git clone ssh://git@{}/{{owner}}/{{name}}`",
                addr.clone(),
                addr.clone()
            )
        }
        server.run_on_address(Arc::new(config), addr).await?;
        Ok(())
    }
}
