use std::net::SocketAddr;
use russh::server::Server;
use crate::metadata::service::MetaService;
use crate::ssh::handler::SshHandler;

pub struct SshServer{
    pub server: MetaService
}


impl Server for SshServer {
    type Handler = SshHandler;

    fn new_client(&mut self, peer_addr: Option<SocketAddr>) -> Self::Handler {
        SshHandler::new(self.server.clone(), peer_addr)
    }
}