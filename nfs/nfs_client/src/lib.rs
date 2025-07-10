use v3::NfsV3Client;
use dashmap::DashMap;
use log::{info, warn};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct NfsMount {
    pub nfs: DashMap<Uuid, NfsVersion>,
}

#[derive(Clone, Debug)]
pub enum NfsVersion {
    V3(NfsV3Client),
    // V4(NfsV4Client),
}

pub mod v3;
pub mod v4;

impl NfsVersion {
    pub async fn mount(&self) -> anyhow::Result<()> {
        match self {
            NfsVersion::V3(nfs) => nfs.mount().await,
        }
    }
    pub async fn umount(self) -> anyhow::Result<()> {
        match self {
            NfsVersion::V3(nfs) => nfs.unmount().await,
        }
    }
    pub async fn is_mount(&self) -> anyhow::Result<bool> {
        match self {
            NfsVersion::V3(nfs) => nfs.is_mounted().await,
        }
    }
}

impl NfsMount {
    pub async fn mount_count(&self) -> usize {
        let mut count = 0;
        for idx in self.nfs.clone() {
            if let Ok(b) = idx.1.is_mount().await {
                if b {
                    count += 1;
                }
            }
        }
        count
    }
    pub async fn mount_list(&self) -> Vec<Uuid> {
        let mut result = vec![];
        let list = self.nfs.clone();
        for idx in list.into_iter() {
            if let Ok(b) = idx.1.is_mount().await {
                if b {
                    result.push(idx.0)
                }
            }
        }
        result
    }
    pub async fn umount_list(&self) -> Vec<Uuid> {
        let mut result = vec![];
        let list = self.nfs.clone();
        for idx in list.into_iter() {
            if let Ok(b) = idx.1.is_mount().await {
                if !b {
                    result.push(idx.0)
                }
            }
        }
        result
    }
    pub async fn new_v3(&mut self, uid: Uuid, config: NfsV3Client) -> &mut Self {
        self.nfs.insert(uid, NfsVersion::V3(config));
        self
    }

    // pub async fn new_v4(&mut self, uid: Uuid, config: NfsV4Client) -> &mut Self {
    //     self.nfs.insert(uid, NfsVersion::V4(config));
    //     self
    // }

    pub async fn map_mount(&self) -> anyhow::Result<()> {
        for (idx, nfs) in self.nfs.clone() {
            match nfs.mount().await {
                Ok(_) => {
                    info!("Mounted {}", idx);
                }
                Err(_) => {
                    warn!("Failed to mount {}", idx);
                }
            }
        }
        Ok(())
    }

    pub async fn map_unmount(&self) -> anyhow::Result<()> {
        for (idx, nfs) in self.nfs.clone() {
            match nfs.umount().await {
                Ok(_) => {
                    info!("Unmounted {}", idx);
                }
                Err(_) => {
                    warn!("Failed to unmount {}", idx);
                }
            }
        }
        Ok(())
    }
}
