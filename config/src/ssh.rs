use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AppSshConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u32,
    pub ed25519_hex: String,
}

impl Default for AppSshConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            host: "0.0.0.0".to_string(),
            port: 30322,
            ed25519_hex: "...".to_string(),
        }
    }
}
