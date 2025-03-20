use crate::handle::SSHandle;
use jz_module::AppModule;
use std::net::SocketAddr;

pub struct SSHServer {
    pub app: AppModule,
}

impl SSHServer {
    pub fn new(app: AppModule) -> Self {
        SSHServer {
            app,
        }
    }
}

impl russh::server::Server for SSHServer {
    type Handler = SSHandle;

    fn new_client(&mut self, _: Option<SocketAddr>) -> Self::Handler {
        SSHandle::new(self.app.clone())
    }
}