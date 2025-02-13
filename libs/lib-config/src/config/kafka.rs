use serde::Deserialize;
use serde::Serialize;

use crate::config::AppConfig;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct KafkaConfig {
    pub addrs : Vec<String>,
}

impl KafkaConfig {
    pub fn new(addrs : Vec<String>) -> Self {
        KafkaConfig { addrs }
    }
}

impl AppConfig {
    pub async fn kafka_config(&self) -> std::io::Result<KafkaConfig> {
        let mut idx = 0;
        let mut result = Vec::new();
        loop {
            let data_id = format!("kafka.{}", idx);
            idx += 1;
            if let Ok(data) = self.client.get_config(data_id, "kafka".to_string()).await {
                result.push(data.content().to_string());
            } else {
                break;
            }
        }
        Ok(KafkaConfig::new(result))
    }
}
