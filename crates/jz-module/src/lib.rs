#![feature(str_as_str)]

use std::time::Duration;
use jz_email::execute::EmailExecute;
use jz_jobs::{Queue, SeaOrmQueue};
use log::info;
use sea_orm::{ConnectOptions, DatabaseConnection};
use jz_dragonfly::Dragonfly;

pub mod container;
pub mod panel;
pub mod repo;
pub mod users;
pub mod email;
pub mod org;
pub mod utils;
pub mod issue;

#[derive(Clone)]
pub struct AppModule {
    pub read: DatabaseConnection,
    pub write: DatabaseConnection,
    pub ioc: container::ContainerIOC,
}

impl AppModule {
    pub async fn init(read: DatabaseConnection, write: DatabaseConnection) -> AppModule {
        let ioc = container::ContainerIOC::init().await;
        AppModule { read, write, ioc }
    }
    pub async fn init_env() -> anyhow::Result<AppModule> {
        // let _ = dotenv::dotenv();
        let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not set");
        info!("Paper Connecting to database:");

        let mut opt = ConnectOptions::new(url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false);
        let read = sea_orm::Database::connect(opt.clone())
            .await
            .expect("Failed to connect to database");
        info!("Read Process Connect Successful!!!");
        let write = sea_orm::Database::connect(opt)
            .await
            .expect("Failed to connect to database");
        info!("Write Process Connect Successful!!!");
        let app = AppModule::init(read, write).await;
        app.init_ioc().await?;
        Ok(app)
    }
    pub async fn init_ioc(&self) -> anyhow::Result<()> {
        info!("Init IOC Container");
        info!("Init Message Queue");
        let queue = SeaOrmQueue::new(self.write.clone(), "backend_jobs".to_string());
        queue.init().await?;
        info!("Init Message Queue Successful");
        self.ioc.inject(queue.clone());
        info!("Init Email Execute");
        let email = EmailExecute::init(queue.clone()).await;
        email.run();
        info!("Init Email Execute Successful");
        self.ioc.inject(email);
        let cache = Dragonfly::connect_pool();
        self.ioc.inject(cache);
        Ok(())
    }
}
