use std::sync::{Arc, Mutex};
use log::info;
use russh::Preferred;
use russh::server::Server;
use russh_keys::PrivateKey;
use russh_keys::ssh_key::private::Ed25519Keypair;
use jzfs::api::service::Service;
use jzfs::config::file::Config;
use jzfs::server::db::init_db;
use jzfs::server::email::init_email;
use jzfs::ssh::server::RusshServer;

#[tokio::main]
async fn main(){
    tracing_subscriber::fmt::init();
    if std::fs::read_dir("./config").is_err() || std::fs::read("./config/id_ed25519").is_err(){
        std::fs::create_dir("./config").ok();
        let ed = Ed25519Keypair::random(&mut rand::rngs::OsRng);
        std::fs::write("./config/id_ed25519", ed.to_bytes()).unwrap();
    }
    let ed = Ed25519Keypair::from_bytes(<&[u8; 64]>::try_from(std::fs::read("./config/id_ed25519").unwrap().as_slice()).unwrap());
    if ed.is_err(){
        std::fs::remove_file("./config/id_ed25519").ok();
        let ed = Ed25519Keypair::random(&mut rand::rngs::OsRng);
        std::fs::write("./config/id_ed25519", ed.to_bytes()).unwrap();
    }
    let ed = Ed25519Keypair::from_bytes(<&[u8; 64]>::try_from(std::fs::read("./config/id_ed25519").unwrap().as_slice()).unwrap()).unwrap();
    
    info!("Git Server Start !!!");
    Config::init().await;
    init_db().await;
    init_email().await;
    let config = russh::server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(3),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![PrivateKey::from(ed)],
        preferred: Preferred {
            ..Preferred::default()
        },
        ..Default::default()
    };
    let config = Arc::new(config);
    let mut sh = RusshServer {
        service: Service::new().await,
    };
    sh.run_on_address(config, ("0.0.0.0", 2222)).await.unwrap();
    
}