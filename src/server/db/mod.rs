use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;
use crate::config::file::CFG;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();


pub async fn init_db(){
    let cfg = CFG.get().unwrap().clone();
    let mut opt = ConnectOptions::new(cfg.db.format());
    // let mut opt = ConnectOptions::new("postgres://postgres:123456@192.168.23.128/jzfs");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    
    let db = Database::connect(opt).await.unwrap();
    DB.get_or_init(||async { 
        db.clone()
    }).await;
}