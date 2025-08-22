use api::AppApiService;
use api::email::email_thread::EmailThread;
use config::AppConfig;
use core::AppCore;
use error::AppError;
use log::{error, warn};
use session::SessionStorage;
use tokio::sync::OnceCell;

pub static NEED_INSTALL: OnceCell<bool> = OnceCell::const_new_with(false);

pub async fn api() -> Result<(), AppError> {
    tracing_subscriber::fmt().init();
    let config = AppConfig::init();
    let _need_install = config == AppConfig::default();

    if _need_install {
        NEED_INSTALL.set(true).ok();
        warn!("Need install");
    }
    let redis = config.redis.conn().await;
    let database = config.database.conn().await;
    let core = AppCore {
        db: database.clone(),
        config: config.clone(),
        redis: redis.clone(),
    };
    core.init_service().await?;
    let session = SessionStorage::new(database.clone(), redis.clone());
    let api = AppApiService {
        core,
        session,
        config: config.clone(),
    };
    EmailThread::init(config.email.clone()).await;
    let git = git::service::GitServer {
        db: database.clone(),
        config,
        redis,
    };
    let git = git::transport::ssh::SSHHandle::new(git);
    tokio::select! {
        r = git.run_ssh() => {
            if let Err(e) = r {
                error!("Error: {}", e);
                std::process::exit(1);
            }
        }
        h = api.run() => {
            if let Err(e) = h {
                error!("Error: {}", e.msg);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}
