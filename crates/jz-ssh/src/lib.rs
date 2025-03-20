use std::sync::Arc;
use russh::keys::PrivateKey;
use russh::keys::ssh_key::private::Ed25519Keypair;
use russh::server::{Config, Server};
use russh::{MethodSet, SshId};
use tracing::info;
use jz_module::AppModule;
use crate::server::SSHServer;

pub mod handle;
pub mod server;

pub struct SSHHandle {
    pub app: AppModule,
}

impl SSHHandle {
    pub fn new(app: AppModule) -> Self {
        Self {
            app,
        }
    }
    pub async fn run_ssh(&self) -> anyhow::Result<()>{
        info!("SSH Starting...");
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
        config.maximum_packet_size = 65535;
        let mut server = SSHServer::new(self.app.clone());
        server.run_on_address(Arc::new(config), "0.0.0.0:30322").await?;
        Ok(())
    }
}