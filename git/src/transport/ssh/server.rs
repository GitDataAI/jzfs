use crate::service::GitServer;
use crate::transport::ssh::handle::SSHandle;
use std::net::SocketAddr;
use tracing::info;

pub struct SSHServer {
    pub app: GitServer,
}

impl SSHServer {
    pub fn new(app: GitServer) -> Self {
        SSHServer { app }
    }
}

impl russh::server::Server for SSHServer {
    type Handler = SSHandle;

    fn new_client(&mut self, addr: Option<SocketAddr>) -> Self::Handler {
        if let Some(addr) = addr {
            info!("New SSH connection from {}", addr);
        }
        SSHandle::new(self.app.clone())
    }
}
