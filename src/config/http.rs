use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct HttpConfig{
    pub port: u16,
    pub host: String,
    pub ssl: bool,
    pub cert: Option<String>,
    pub key: Option<String>,
    pub server: String,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self{
            port: 80,
            host: "0.0.0.0".to_string(),
            ssl: false,
            cert: None,
            key: None,
            server: "actix".to_string(),
        }
    }
}

impl HttpConfig {
    pub fn format(&self) -> String{
        match self.ssl{
            true => format!("https://{}:{}", self.host, self.port),
            false => format!("http://{}:{}", self.host, self.port),
        }
    }
    pub fn starter(&self) -> String{
        format!("{}:{}", self.host, self.port)
    }
}