use nacos_sdk::api::config::{ConfigService, ConfigServiceBuilder};
use nacos_sdk::api::naming::{NamingService, NamingServiceBuilder, ServiceInstance};
use nacos_sdk::api::props::ClientProps;

pub struct AppNaCos {
    pub client: ClientProps,
    pub naming: NamingService,
    pub config: ConfigService,
}

impl AppNaCos {
    pub fn from_env() -> anyhow::Result<Self> {
        let nacos_addr = std::env::var("NACOS_ADDR").expect("NACOS_ADDR not set");
        let nacos_user = std::env::var("NACOS_USER").expect("NACOS_USER not set");
        let nacos_pass = std::env::var("NACOS_PASS").expect("NACOS_PASS not set");
        let nacos_namespace = std::env::var("NACOS_NAMESPACE").unwrap_or("public".to_string());
        let nacos_app_name = std::env::var("NACOS_APP_NAME").expect("NACOS_APP_NAME not set");
        let client = ClientProps::new()
            .app_name(nacos_app_name)
            .auth_username(nacos_user)
            .auth_password(nacos_pass)
            .server_addr(nacos_addr)
            .namespace(nacos_namespace)
            .max_retries(3);
        let naming = NamingServiceBuilder::new(client.clone())
            .enable_auth_plugin_http()
            .build()?;
        let config = ConfigServiceBuilder::new(client.clone())
            .enable_auth_plugin_http()
            .build()?;
        Ok(Self {
            client,
            naming,
            config,
        })
    }
    pub async fn get_service_list(&self, service_name: &str, group_name: Option<String>) -> anyhow::Result<Vec<ServiceInstance>> {
        let services = self.naming.get_all_instances(service_name.to_string(),group_name,vec![], true).await?;
        Ok(services)
    }
    pub async fn rand_get_service(&self, service_name: &str, group_name: Option<String>) -> anyhow::Result<ServiceInstance> {
        let services = self.get_service_list(service_name, group_name).await?;
        let index = (rand::random::<i32>() % services.len() as i32) as usize;
        let service = services.get(index).ok_or(anyhow::anyhow!("no service"))?;
        Ok(service.clone())
    }
    pub async fn get_config(&self, group_name: String, data_id: String) -> anyhow::Result<String> {
        let config = self.config.get_config(data_id, group_name).await?;
        Ok(config.content().clone())
    }



}