// use std::sync::{Arc, Mutex};
// use log::info;
// use tokio_nsq::{NSQChannel, NSQProducer, NSQTopic};
// use lib_config::AppNacos;
// use rand::{Rng};
// 
// pub struct AppNQLClient {
//     producer: Arc<Mutex<NSQProducer>>,
//     topic: Arc<NSQTopic>,
//     _channel: Arc<NSQChannel>
// }
// 
// impl AppNQLClient {
//     pub async fn init(nacos: AppNacos, topic: String, channel: String) -> std::io::Result<Self>{
//         let config = nacos.config;
//         let nsq_config = config.nsq_config().await?;
//         let mut address = Vec::new();
//         for addr in nsq_config.addrs {
//             address.push(addr);
//         }
//         let tc = NSQTopic::new(&topic).unwrap();
//         let ch = NSQChannel::new(&channel).unwrap();
//         info!("nsq server init topic: {} channel: {}", topic, channel);
//         let mut rng = rand::thread_rng();
//         let addr = address[rng.gen_range(0, address.len())].clone();
//         let consumer = tokio_nsq::NSQProducerConfig::new(
//             addr
//         )
//             .build();
//         Ok(Self {
//             producer: Arc::new(Mutex::new(consumer)),
//             topic: tc,
//             _channel: ch
//         })
//     }
//     pub async fn send(&self, message: Vec<u8>) -> std::io::Result<()>{
//         self.producer.lock().unwrap().publish(&self.topic, message).await.ok();
//         Ok(())
//     }
// }
// 
// #[cfg(test)]
// mod tests {
//     use lib_config::AppNacos;
//     use crate::{CHANNEL, EMAIL_TOPIC};
//     use crate::server::email::EmailType;
//     use super::*;
//     #[tokio::test]
//     async fn test_nql_client() {
//         let nacos = AppNacos::from_env().unwrap(); 
//         let client = AppNQLClient::init(nacos, EMAIL_TOPIC.to_string(), CHANNEL.to_string()).await.unwrap();
//         client.send(serde_json::to_vec(&EmailType::Captcha).expect("REASON")).await.unwrap();
//         tokio::time::sleep(std::time::Duration::from_secs(5)).await;
//     }
// }
