use nacos_sdk::api::config::ConfigService;
use nacos_sdk::api::naming::{NamingService, ServiceInstance};
use std::sync::Arc;
use crate::config::Config;
use crate::config::email::EmailConfig;
use crate::config::http::HttpConfig;
use crate::config::mongodb::MongoDBConfig;
use crate::config::postgres::PgConfig;
use crate::config::redis::RedisConfig;

pub struct NacosClient {
    config: Arc<Box<dyn ConfigService>>,
    naming: Arc<Box<dyn NamingService>>,
}


impl NacosClient {
    pub fn new(config: Arc<Box<dyn ConfigService>>, naming: Arc<Box<dyn NamingService>>) -> Self {
        Self { config, naming }
    }
    pub async fn connect(app_name: String) -> Self {
        let addr = std::env::var("NACOS_ADDR").expect("NACOS_ADDR NotFound Please setting env");
        let username = std::env::var("NACOS_USER").expect("NACOS_USER NotFound Please setting env");
        let password =
            std::env::var("NACOS_PASSWORD").expect("NACOS_PASSWORD not found please setting env");
        let namespace =
            std::env::var("NACOS_NAMESPACE").expect("NACOS_NAMESPACE not found please setting env");
        let props = nacos_sdk::api::props::ClientProps::new()
            .server_addr(addr)
            .namespace(namespace)
            .auth_username(username)
            .auth_password(password)
            .app_name(app_name)
            .max_retries(5);
        let config_service = nacos_sdk::api::config::ConfigServiceBuilder::new(props.clone())
            .enable_auth_plugin_http()
            .build()
            .expect("Nacos Config Service Connect Err");
        let naming_service = nacos_sdk::api::naming::NamingServiceBuilder::new(props)
            .enable_auth_plugin_http()
            .build()
            .expect("Nacos Naming Service Connect Err");
        Self {
            config: Arc::new(Box::new(config_service)),
            naming: Arc::new(Box::new(naming_service)),
        }
    }
    pub async fn email(&self) -> EmailConfig {
        let email = self
            .config
            .get_config("email.yaml".to_string(), "DEFAULT_GROUP".to_string())
            .await
            .expect("Nacos Config Service Connect Err");
        let email: EmailConfig = serde_yaml::from_str(&email.content()).expect("Nacos Config Service Connect Err");
        email
    }
    pub async fn http(&self) -> HttpConfig {
        let http = self
            .config
            .get_config("http.yaml".to_string(), "DEFAULT_GROUP".to_string())
            .await
            .expect("Nacos Config Service Connect Err");
        let http: HttpConfig = serde_yaml::from_str(&http.content()).expect("Nacos Config Service Connect Err");
        http
    }
    pub async fn postgres(&self) -> PgConfig {
        let postgres = self
            .config
            .get_config("postgres.yaml".to_string(), "DEFAULT_GROUP".to_string())
            .await
            .expect("Nacos Config Service Connect Err");
        let postgres: PgConfig = serde_yaml::from_str(&postgres.content()).expect("Nacos Config Service Connect Err");
        postgres
    }
    pub async fn mongodb(&self) -> MongoDBConfig {
        let mongodb = self
            .config
            .get_config("mongodb.yaml".to_string(), "DEFAULT_GROUP".to_string())
            .await
            .expect("Nacos Config Service Connect Err");
        let mongodb: MongoDBConfig = serde_yaml::from_str(&mongodb.content()).expect("Nacos Config Service Connect Err");
        mongodb
    }
    pub async fn redis(&self) -> RedisConfig {
        let redis = self
            .config
            .get_config("redis.yaml".to_string(), "DEFAULT_GROUP".to_string())
            .await
            .expect("Nacos Config Service Connect Err");
        let redis: RedisConfig = serde_yaml::from_str(&redis.content()).expect("Nacos Config Service Connect Err");
        redis
    }
    pub async fn all(&self) -> anyhow::Result<Config> { 
        let config = Config {
            email: self.email().await,
            http: self.http().await,
            postgres: self.postgres().await,
            redis: self.redis().await,
            mongodb: self.mongodb().await,
        };
        Ok(config)
    }
    pub async fn register_http(&self) -> anyhow::Result<()> {
        let mut instance = ServiceInstance::default();
        let api = self.http().await;
        let ips = sysinfo::Networks::new_with_refreshed_list()
            .iter()
            .map(|(_,x)|{
                x.ip_networks()
                    .iter()
                    .filter(|x|x.addr.is_ipv4())
                    .map(|x|x.addr.to_string())
                    .filter(|x|!x.is_empty())
                    .collect::<Vec<String>>()
                    .join(",")
            })
            .filter(|x|!x.is_empty())
            .collect::<Vec<String>>()
            .join(",");
        instance.ip = ips;
        instance.port = api.port as i32;
        self.naming
            .register_instance(
                "http".to_string(),
                Some("DEFAULT_GROUP".to_string()),
                instance
            )
            .await?;
        Ok(())
    }
}