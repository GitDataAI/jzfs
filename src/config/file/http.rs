use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct HttpConfig {
    pub port: String,
    pub host: String,
    pub worker: usize,
}


impl Default for HttpConfig {
    fn default() -> Self {
        Self{
            port: "80".to_string(),
            host: "0.0.0.0".to_string(),
            worker: 8,
        }
    }
}

impl HttpConfig {
    pub fn format(&self) -> String{
        format!("{}:{}",
            self.host,
            self.port
        )
    }
    pub fn worker(&self) -> usize{
        self.worker
    }
}