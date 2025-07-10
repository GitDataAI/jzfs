use dashmap::DashMap;
use nfs3_client::tokio::{TokioConnector, TokioIo};
use nfs3_client::{Nfs3Connection, Nfs3ConnectionBuilder};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, OnceCell};
use uuid::Uuid;

pub static HANDLE_POOL: OnceCell<Mutex<DashMap<Uuid, Nfs3Connection<TokioIo<TcpStream>>>>> =
    OnceCell::const_new();

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NfsV3Client {
    pub uid: Uuid,
    pub host: String,
    pub nfs_port: u16,
    pub mount_port: u16,
    pub endpoint: String,
}

impl NfsV3Client {
    pub async fn mount(&self) -> anyhow::Result<()> {
        let connection =
            Nfs3ConnectionBuilder::new(TokioConnector, self.host.clone(), self.endpoint.clone())
                .nfs3_port(self.nfs_port)
                .mount_port(self.mount_port)
                .mount()
                .await?;
        if let Some(pool) = HANDLE_POOL.get() {
            let lock = pool.lock().await;
            lock.insert(self.uid, connection);
        } else {
            let pool = DashMap::new();
            pool.insert(self.uid, connection);
            let pool = Mutex::new(pool);
            HANDLE_POOL.get_or_init(|| async { pool }).await;
        }
        Ok(())
    }
    pub async fn unmount(&self) -> anyhow::Result<()> {
        if let Some(pool) = HANDLE_POOL.get() {
            let lock = pool.lock().await;
            if let Some((_, handle)) = lock.remove(&self.uid) {
                handle.unmount().await?;
            }
        } else {
            return Err(anyhow::anyhow!("not mounted"));
        }
        Ok(())
    }
    pub async fn is_mounted(&self) -> anyhow::Result<bool> {
        if let Some(pool) = HANDLE_POOL.get() {
            let lock = pool.lock().await;
            if let Some(_) = lock.get(&self.uid) {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(anyhow::anyhow!("not mounted"))
        }
    }
}
