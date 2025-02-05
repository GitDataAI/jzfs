use std::time::Duration;
use log::{info, warn};
use rdkafka::ClientConfig;
use lib_config::AppNacos;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppKafkaClient {
    producer: FutureProducer,
}

impl AppKafkaClient {
    pub async fn init(nacos: AppNacos, group: String) -> std::io::Result<Self>{
        let config = nacos.config.kafka_config().await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
            .addrs.first()
            .map(|s| s.to_string())
            .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "kafka config error"))?;
        let producer:FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", config)
            .set("group.id", &group)
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", "earliest")
            .create()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        info!("kafka producer init success");
        Ok(AppKafkaClient {
            producer
        })
    }
    pub async fn send(&self, topic: String, key: Option<String> ,message: Vec<u8>) -> std::io::Result<()>{
        let key = key.unwrap_or(Uuid::new_v4().to_string());
        let record = FutureRecord::to(&topic)
            .key(key.as_bytes())
            .payload(&message);
        self.producer.send(record, Timeout::After(Duration::from_secs(60))).await
            .map_err(|e| {
                warn!("kafka send error:{}", e.0.to_string());
                std::io::Error::new(std::io::ErrorKind::Other, e.0.to_string())
            })?;
        Ok(())
    }
}

