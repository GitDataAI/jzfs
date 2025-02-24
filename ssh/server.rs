use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use tracing::info;
use crate::services::{AppState, AppStateHandle};
use crate::ssh::handle::SSHandle;


pub struct SSHServer {
    pub app: AppState,
}

impl SSHServer {
    pub fn new(app: AppState) -> Self {
        SSHServer {
            app,
        }
    }
    pub async fn from_env() -> io::Result<Self> {
        let app = AppStateHandle::get().await;
        Ok(SSHServer::new(app))
    }
}

impl russh::server::Server for SSHServer {
    type Handler = SSHandle;

    fn new_client(&mut self, _: Option<SocketAddr>) -> Self::Handler {
        info!("New Client");
        SSHandle {
            app: self.app.clone(),
            stdin: HashMap::new(),
            user: None,
        }
    }
}