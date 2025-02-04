use std::sync::Arc;

#[derive(Clone)]
pub struct AppConfig {
    pub client: Arc<Box<dyn nacos_sdk::api::config::ConfigService>>
}

impl AppConfig {
    pub fn new(client: Arc<Box<dyn nacos_sdk::api::config::ConfigService>>) -> Self {
        AppConfig {
            client
        }
    }
}

pub mod postgres;
pub mod redis;
pub mod auth;