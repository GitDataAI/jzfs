use crate::api::AppApiConfig;
use crate::database::AppDatabaseConfig;
use crate::email::AppEmailConfig;
use crate::git::AppGitConfig;
use crate::redis::AppRedisConfig;
use crate::ssh::AppSshConfig;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Default, Eq, PartialEq)]
pub struct AppConfig {
    #[serde(rename = "database")]
    pub database: AppDatabaseConfig,
    #[serde(rename = "redis")]
    pub redis: AppRedisConfig,
    #[serde(rename = "git")]
    pub git: AppGitConfig,
    #[serde(rename = "api")]
    pub api: AppApiConfig,
    #[serde(rename = "email")]
    pub email: AppEmailConfig,
    #[serde(rename = "ssh")]
    pub ssh: AppSshConfig,
}

pub mod api;
pub mod database;
pub mod email;
pub mod git;
pub mod redis;
pub mod ssh;

impl AppConfig {
    pub fn new() -> Self {
        AppConfig {
            database: AppDatabaseConfig::default(),
            redis: AppRedisConfig::default(),
            git: AppGitConfig::default(),
            api: AppApiConfig::new(),
            email: AppEmailConfig::default(),
            ssh: Default::default(),
        }
    }
    pub fn init() -> AppConfig {
        let mut config_file = std::env::var("CONFIG_FILE").unwrap_or("config.toml".to_string());
        if !config_file.ends_with(".toml") {
            config_file.push_str(".toml");
        }
        if std::path::Path::new(&config_file).exists() {
            let config_file =
                std::fs::read_to_string(config_file).expect("Failed to read config file");
            let config: AppConfig =
                toml::from_str(&config_file).expect("Failed to parse config file");
            config
        } else {
            println!("Config file not found, using default config");
            let buf = toml::to_string(&AppConfig::default()).expect("Failed to serialize config");
            std::fs::write(config_file, buf).expect("Failed to write config file");
            Self::default()
        }
    }
}

#[test]
fn init_config() {
    AppConfig::init();
}
