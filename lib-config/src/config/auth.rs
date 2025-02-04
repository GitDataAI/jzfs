use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct AppApiAuthConfig {
    pub port: i32,
}