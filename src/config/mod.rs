use crate::config::http::HttpConfig;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use tracing::{error, info};

pub mod email;
pub mod http;
pub mod mongodb;
pub mod postgres;
pub mod redis;

pub static CFG: OnceCell<Config> = OnceCell::const_new();

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Config {
    pub http: HttpConfig,
    pub postgres: postgres::PgConfig,
    pub redis: redis::RedisConfig,
    pub mongodb: mongodb::MongoDBConfig,
    pub email: email::EmailConfig,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_file = std::fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml::from_str(&config_file)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_file = serde_yaml::to_string(&config)?;
    std::fs::write("config.yaml", config_file)?;
    Ok(())
}

pub fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();
    match config {
        Ok(config) => CFG.set(config)?,
        Err(_) => {
            error!("Config file not found, using default config");
            let config = Config::default();
            save_config(&config)?;
            let current_dir = std::env::current_dir()?;
            error!(
                "Config file created at {}",
                current_dir.join("config.yaml").display()
            );
            error!("Please edit the config file and restart the server");
            std::process::exit(0);
        }
    }
    info!("Config loaded");
    Ok(())
}
