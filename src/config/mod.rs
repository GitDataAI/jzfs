use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use tracing::{error, info};
use crate::config::http::HttpConfig;

pub mod http;
pub mod postgres;
pub mod redis;
pub mod mongodb;
pub mod email;

pub static CFG: OnceCell<Config> = OnceCell::const_new();


#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Config{
    pub http: HttpConfig,
    pub postgres: postgres::PgConfig,
    pub redis: redis::RedisConfig,
    pub mongodb: mongodb::MongoDBConfig,
    pub email: email::EmailConfig,
}

impl Default for Config{
    fn default() -> Self{
        Config{
            http: HttpConfig::default(),
            postgres: postgres::PgConfig::default(),
            redis: redis::RedisConfig::default(),
            mongodb: mongodb::MongoDBConfig::default(),
            email: email::EmailConfig::default(),
        }
    }
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>>{
    let config_file = std::fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml::from_str(&config_file)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>>{
    let config_file = serde_yaml::to_string(&config)?;
    std::fs::write("config.yaml", config_file)?;
    Ok(())
}

pub fn init_config() -> Result<(), Box<dyn std::error::Error>>{
    let config = load_config();
    match config{
        Ok(config) => CFG.set(config)?,
        Err(_) => {
            error!("Config file not found, using default config");
            let config = Config::default();
            save_config(&config)?;
            let current_dir = std::env::current_dir()?;
            error!("Config file created at {}", current_dir.join("config.yaml").display());
            error!("Please edit the config file and restart the server");
            std::process::exit(0);
        },
    }
    info!("Config loaded");
    Ok(())
}