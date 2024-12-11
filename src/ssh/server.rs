use crate::api::service::Service;
use crate::ssh::config::RusshServerConfig;
use crate::ssh::handle::RusshServerHandler;
use russh::server::Server;
use std::net::SocketAddr;

pub struct RusshServerInternals {
    pub config: RusshServerConfig,
}
#[derive(Clone)]
pub struct RusshServer {
    pub service: Service,
}
impl Server for RusshServer {
    type Handler = RusshServerHandler;

    fn new_client(&mut self, peer_addr: Option<SocketAddr>) -> Self::Handler {
        RusshServerHandler::new(self, peer_addr)
    }
}
