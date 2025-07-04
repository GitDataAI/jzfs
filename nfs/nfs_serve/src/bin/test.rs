use std::path::PathBuf;
use nfs_serve::nfs_serve::Storage;
use nfs_serve::tcp::{NFSTcp, NFSTcpListener};

const HOSTPORT: u32 = 2049;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .init();
    let listener = NFSTcpListener::bind(
        &format!("0.0.0.0:{HOSTPORT}"),
        Storage::new(PathBuf::from("E:\\nfs")).unwrap(),
    )
        .await
        .unwrap();
    listener.handle_forever().await.ok();
}
// mount -t nfs -o nolocks,vers=3,tcp,port=11111,mountport=11111,soft 172.29.112.1:/ mnt/
