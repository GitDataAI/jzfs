use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

pub mod db;
pub mod http;
pub mod redis;
pub mod email;

const CONFIG_FILE_NAME: &str = "./config/config.toml";
pub static CFG:OnceCell<Config> = OnceCell::const_new();
#[derive(Deserialize,Serialize,Default,Clone,Debug)]
pub struct Config{
    pub db: db::DBConfig,
    pub http: http::HttpConfig,
    pub redis: redis::RedisConfig,
    pub email: email::EmailConfig,
}
impl Config {
    pub async fn init() -> Self{
        if std::fs::read_dir("./config").is_err(){
            std::fs::create_dir("./config").unwrap()
        }
        if std::fs::read(CONFIG_FILE_NAME).is_err(){
            if std::env::var("CONFIG").is_ok(){
                let config:Config = toml::from_str(&std::env::var("CONFIG").unwrap()).unwrap();
                std::fs::write(CONFIG_FILE_NAME, toml::to_string(&config).unwrap()).unwrap();
                return config;
            }
            let config = Config::default();
            let config = toml::to_string(&config).unwrap();
            std::fs::write(CONFIG_FILE_NAME, config).unwrap();
            return Config::default();
        }
        let cfg:Config = toml::from_str(&std::fs::read_to_string(CONFIG_FILE_NAME).unwrap()).unwrap();
        CFG.get_or_init(||async {
            cfg.clone()
        }).await;
        cfg
    }
}
