use crate::context::RPCContext;
use crate::rpcwire::*;
use crate::transaction_tracker::TransactionTracker;
use crate::vfs::NFSFileSystem;
use anyhow;
use async_trait::async_trait;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use std::{io, net::IpAddr};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

pub struct NFSTcpListener<T: NFSFileSystem + Send + Sync + 'static> {
    listener: TcpListener,
    port: u16,
    arcfs: Arc<T>,
    mount_signal: Option<mpsc::Sender<bool>>,
    export_name: Arc<String>,
    transaction_tracker: Arc<TransactionTracker>,
}

pub fn generate_host_ip(hostnum: u16) -> String {
    format!(
        "127.88.{}.{}",
        ((hostnum >> 8) & 0xFF) as u8,
        (hostnum & 0xFF) as u8
    )
}
async fn process_socket(
    mut socket: tokio::net::TcpStream,
    context: RPCContext,
) -> Result<(), anyhow::Error> {
    let (mut message_handler, mut socksend, mut msgrecvchan) = SocketMessageHandler::new(&context);
    let _ = socket.set_nodelay(true);
    tokio::spawn(async move {
        loop {
            if let Err(e) = message_handler.read().await {
                debug!("Message loop broken due to {:?}", e);
                break;
            }
        }
    });
    loop {
        tokio::select! {
            _ = socket.readable() => {
                let mut buf = [0; 128000];

                match socket.try_read(&mut buf) {
                    Ok(0) => {
                        return Ok(());
                    }
                    Ok(n) => {
                        let _ = socksend.write_all(&buf[..n]).await;
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        debug!("Message handling closed : {:?}", e);
                        return Err(e.into());
                    }
                }

            },
            reply = msgrecvchan.recv() => {
                match reply {
                    Some(Err(e)) => {
                        debug!("Message handling closed : {:?}", e);
                        return Err(e);
                    }
                    Some(Ok(msg)) => {
                        if let Err(e) = write_fragment(&mut socket, &msg).await {
                            error!("Write error {:?}", e);
                        }
                    }
                    None => {
                        return Err(anyhow::anyhow!("Unexpected socket context termination"));
                    }
                }
            }
        }
    }
}

#[async_trait]
pub trait NFSTcp: Send + Sync {
    fn get_listen_port(&self) -> u16;
    fn get_listen_ip(&self) -> IpAddr;
    fn set_mount_listener(&mut self, signal: mpsc::Sender<bool>);
    async fn handle_forever(&self) -> io::Result<()>;
}

impl<T: NFSFileSystem + Send + Sync + 'static> NFSTcpListener<T> {
    pub async fn bind(ipstr: &str, fs: T) -> io::Result<NFSTcpListener<T>> {
        let (ip, port) = ipstr.split_once(':').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::AddrNotAvailable,
                "IP Address must be of form ip:port",
            )
        })?;
        let port = port.parse::<u16>().map_err(|_| {
            io::Error::new(
                io::ErrorKind::AddrNotAvailable,
                "Port not in range 0..=65535",
            )
        })?;

        let arcfs: Arc<T> = Arc::new(fs);
        if ip == "auto" {
            let mut num_tries_left = 32;

            for try_ip in 1u16.. {
                let ip = generate_host_ip(try_ip);

                let result = NFSTcpListener::bind_internal(&ip, port, arcfs.clone()).await;

                match &result {
                    Err(_) => {
                        if num_tries_left == 0 {
                            return result;
                        } else {
                            num_tries_left -= 1;
                            continue;
                        }
                    }
                    Ok(_) => {
                        return result;
                    }
                }
            }
            unreachable!(); // Does not detect automatically that loop above never terminates.
        } else {
            NFSTcpListener::bind_internal(ip, port, arcfs).await
        }
    }

    async fn bind_internal(ip: &str, port: u16, arcfs: Arc<T>) -> io::Result<NFSTcpListener<T>> {
        let ipstr = format!("{ip}:{port}");
        let listener = TcpListener::bind(&ipstr).await?;
        info!("Listening on {:?}", &ipstr);

        let port = match listener.local_addr().unwrap() {
            SocketAddr::V4(s) => s.port(),
            SocketAddr::V6(s) => s.port(),
        };
        Ok(NFSTcpListener {
            listener,
            port,
            arcfs,
            mount_signal: None,
            export_name: Arc::from("/".to_string()),
            transaction_tracker: Arc::new(TransactionTracker::new(Duration::from_secs(60))),
        })
    }
    pub fn with_export_name<S: AsRef<str>>(&mut self, export_name: S) {
        self.export_name = Arc::new(format!(
            "/{}",
            export_name
                .as_ref()
                .trim_end_matches('/')
                .trim_start_matches('/')
        ))
    }
}

#[async_trait]
impl<T: NFSFileSystem + Send + Sync + 'static> NFSTcp for NFSTcpListener<T> {
    fn get_listen_port(&self) -> u16 {
        let addr = self.listener.local_addr().unwrap();
        addr.port()
    }

    fn get_listen_ip(&self) -> IpAddr {
        let addr = self.listener.local_addr().unwrap();
        addr.ip()
    }

    fn set_mount_listener(&mut self, signal: mpsc::Sender<bool>) {
        self.mount_signal = Some(signal);
    }

    async fn handle_forever(&self) -> io::Result<()> {
        loop {
            let (socket, _) = self.listener.accept().await?;
            let context = RPCContext {
                local_port: self.port,
                client_addr: socket.peer_addr()?.to_string(),
                auth: crate::rpc::auth_unix::default(),
                vfs: self.arcfs.clone(),
                mount_signal: self.mount_signal.clone(),
                export_name: self.export_name.clone(),
                transaction_tracker: self.transaction_tracker.clone(),
            };
            info!("Accepting connection from {}", context.client_addr);
            debug!("Accepting socket {:?} {:?}", socket, context);
            tokio::spawn(async move {
                let _ = process_socket(socket, context).await;
            });
        }
    }
}
