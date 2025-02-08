use std::collections::HashMap;
use crate::naming::key::NamingKey;
use chrono::Utc;
use nacos_sdk::api::naming::ServiceInstance;
use std::sync::Arc;
use log::error;
use serde::{Deserialize, Serialize};

pub mod key;


#[derive(Clone)]
pub struct AppNaming {
    pub client: Arc<Box<dyn nacos_sdk::api::naming::NamingService>>,
    pub name: String,
    pub name_key: Option<NamingKey>
}

impl AppNaming {
    pub fn new(client: Arc<Box<dyn nacos_sdk::api::naming::NamingService>>,name: String) -> Self {
        AppNaming {
            client,
            name,
            name_key: None,
        }
    }
    pub async fn register(&mut self, port: i32, groups: &str, kind: i32) -> anyhow::Result<Self> {
        let mut key = NamingKey::default();
        key.set_kind(kind);
        key.set_naming(self.name.clone());
        key.set_port(port);
        key.set_startime(Utc::now().timestamp());
        let ips = sysinfo::Networks::new_with_refreshed_list()
            .iter()
            .map(|x| x.1)
            .map(|x|{
                x.ip_networks()
                    .iter()
                    .map(|x|x.addr)
                    .filter(|x|x.is_ipv4())
                    .map(|x|x.to_string())
                    .filter(|x|!x.is_empty())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect::<Vec<String>>();

        key.set_ips(ips.clone());
        let mut instance = ServiceInstance::default();
        instance.port = port;
        instance.ip = ips.join(",");
        instance.service_name = Some(self.name.clone());
        self.client.register_instance(
            key.to_string(),
            Some(groups.to_string()),
            instance,
        ).await?;
        self.name_key = Some(key);
        Ok(self.clone())
    }

    pub async fn unregister(&self) -> anyhow::Result<()> {
        if let Some(key) = &self.name_key {
            self.client.deregister_instance(
                key.to_string(),
                None,
                ServiceInstance::default(),
            ).await?;
        }

        Ok(())
    }
    pub async fn list_http_server(&self, group: Vec<&str>) -> anyhow::Result<Vec<HttpServiceNode>> {
        let mut result = HashMap::new();
        for idx in group {
            let (res, _) = self.client
                .get_service_list(
                    0,
                    i32::MAX,
                    Some(idx.to_string())
                ).await?;
            result.insert(idx,res);
        }
        let mut ins = Vec::new();
        for (group, idx) in result {
            for idx in idx {
                match self.client
                    .get_all_instances(
                        idx.to_string(),
                        Some(group.to_string().clone()),
                        vec![],
                        false,
                    )
                    .await{
                    Ok(mut x) => {
                        ins.extend_from_slice(
                            x
                                .iter_mut()
                                .map(|x|{
                                    let name = x.service_name.clone().unwrap_or("@@none".to_string())
                                        .split("@@")
                                        .collect::<Vec<_>>()
                                        .last()
                                        .map(|x|x.to_string())
                                        .unwrap_or("none".to_string());
                                    x.service_name = Some(name);
                                    let ips = x.ip.split(",")
                                        .map(|x|x.to_string())
                                        .collect::<Vec<_>>()
                                        .iter()
                                        .filter(|x|!x.is_empty())
                                        .filter(|x|!x.contains("127.0.0.1"))
                                        .map(|x|x.to_string())
                                        .collect::<Vec<_>>()
                                        .join(",");
                                    x.ip = ips;
                                    x
                                })
                                .map(|x|x.clone())
                                .collect::<Vec<_>>()
                                .as_slice()
                        );
                    },
                    Err(err) => {
                        error!("{}", err);
                        continue;
                    }
                }
            }

        }
        let result = ins
            .iter()
            .map(|x|{
                HttpServiceNode{
                    endpoint: x.service_name.clone().unwrap(),
                    ips: x.ip.split(",").map(|x|x.to_string()).collect::<Vec<_>>(),
                    port: x.port,
                }
            })
            .collect::<Vec<_>>();
        Ok(result)
    }
}


#[derive(Clone,Debug,Deserialize,Serialize,PartialEq,Hash,Eq)]
pub struct HttpServiceNode {
    pub endpoint: String,
    pub ips: Vec<String>,
    pub port: i32,
}

#[cfg(test)]
mod tests {
    use crate::AppNacos;

    #[tokio::test]
    async fn test_list() {
        let nacos = AppNacos::from_env().unwrap();
        let result = nacos.naming.list_http_server(vec!["api","web"]).await.unwrap();
        dbg!(result);
    }
}
