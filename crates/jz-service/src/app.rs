use log::info;
use sqlx::postgres::PgPoolOptions;
use jz_dragonfly::Dragonfly;
use jz_email::execute::EmailExecute;
use jz_jobs::{Queue, QueueJobs};
use jz_jobs::sqlx::SqlxQueue;
use crate::container;

pub struct AppService {
    pub read: sqlx::PgPool,
    pub write: sqlx::PgPool,
    pub ioc: container::ContainerIOC,
}

impl AppService {
    pub async fn init(read: sqlx::PgPool, write: sqlx::PgPool) -> AppService {
        let ioc = container::ContainerIOC::init();
        AppService { read, write, ioc }
    }
    pub async fn init_env() -> anyhow::Result<AppService> {
        let _ = dotenv::dotenv();
        let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not set");
        info!("Paper Connecting to database:");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url).await?;
        let read = pool.clone();
        info!("Read Process Connect Successful!!!");
        let write = pool.clone();
        info!("Write Process Connect Successful!!!");
        let app = AppService::init(read, write).await;
        app.init_ioc().await?;
        Ok(app)
    }
    pub async fn init_ioc(&self) -> anyhow::Result<()> {
        info!("Init IOC Container");
        info!("Init Message Queue");
        let queue = SqlxQueue::init(self.write.clone(), "backend_jobs".to_string());
        queue.init().await?;
        info!("Init Message Queue Successful");
        self.ioc.inject(queue.clone());
        info!("Init Email Execute");
        let queue = QueueJobs::new_sqlx(queue);
        let email = EmailExecute::init(queue.clone()).await;
        email.run();
        info!("Init Email Execute Successful");
        self.ioc.inject(email);
        let cache = Dragonfly::connect_pool();
        self.ioc.inject(cache);
        Ok(())
    }
}