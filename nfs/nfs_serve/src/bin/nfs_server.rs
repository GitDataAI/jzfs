use nfs_serve::nfs_serve::Storage;
use nfs_serve::tcp::{NFSTcp, NFSTcpListener};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let hostport: u16 = std::env::var("NFS_PORT").unwrap_or("2049".to_string()).parse().expect("NFS_PORT Shoud is a number");
    tracing_subscriber::fmt().init();
    if std::fs::read_dir("./data").is_err() {
        std::fs::create_dir("./data").unwrap();
    }
    let listener = NFSTcpListener::bind(
        &format!("0.0.0.0:{hostport}"),
        Storage::new(PathBuf::from("./data")).unwrap(),
    )
    .await
    .unwrap();
    tracing::info!("NFS Server Started");
    listener.handle_forever().await.ok();
}
// mount -t nfs -o nolocks,vers=3,tcp,port=11111,mountport=11111,soft 172.29.112.1:/ mnt/
