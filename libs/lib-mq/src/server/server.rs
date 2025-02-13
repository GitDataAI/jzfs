use std::sync::Arc;

use lib_config::AppNacos;
use log::error;
use log::info;
use rdkafka::ClientConfig;
use rdkafka::Message;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;

use crate::server::email::EmailEvent;
use crate::server::email::EmailType;
use crate::server::topic::Topic;

#[derive(Clone)]
pub struct AppKafkaConsumer {
    consumer : Arc<StreamConsumer>,
    nacos : AppNacos,
}

impl AppKafkaConsumer {
    pub async fn init(nacos : AppNacos, channel : String) -> std::io::Result<Self> {
        let config = nacos
            .config
            .kafka_config()
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
            .addrs
            .first()
            .map(|s| s.to_string())
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "kafka config error",
            ))?;
        let consumer : StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", config)
            .set("group.id", &channel)
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", "earliest")
            .create()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        info!("kafka consumer init success group: {}", channel);
        Ok({
            AppKafkaConsumer {
                consumer : Arc::new(consumer),
                nacos,
            }
        })
    }
    pub async fn topic(&mut self, topics : Vec<String>) -> std::io::Result<()> {
        let consumer = self.consumer.clone();
        consumer
            .subscribe(&topics.iter().map(|x| x.as_str()).collect::<Vec<&str>>())
            .map_err(|x| std::io::Error::new(std::io::ErrorKind::Other, x.to_string()))?;
        self.consumer = consumer;
        Ok(())
    }
    pub async fn listen(self) {
        let consumer = self.consumer.clone();
        tokio::spawn(async move {
            let email = EmailEvent::new(self.nacos.clone())
                .await
                .map_err(|e| {
                    error!("{}", e);
                })
                .expect("email event init error");
            loop {
                match consumer.recv().await {
                    Ok(message) => {
                        let topic = message.topic();
                        let payload = message.payload().unwrap_or(b"").to_vec();
                        match Topic::try_from_string(topic.to_string()) {
                            Some(topic) => {
                                info!(
                                    "Received new work from {}, ready to start execution",
                                    topic.to_string()
                                );
                                match topic {
                                    Topic::Email => {
                                        if let Ok(email_type) =
                                            serde_json::from_slice::<EmailType>(&payload)
                                        {
                                            email.send(email_type).await;
                                            consumer
                                                .commit_message(
                                                    &message,
                                                    rdkafka::consumer::CommitMode::Sync,
                                                )
                                                .ok();
                                        }
                                    }
                                }
                            }
                            None => continue,
                        }
                    }
                    Err(e) => {
                        error!("Kafka error: {}", e);
                        continue;
                    }
                }
            }
        });
    }
}
