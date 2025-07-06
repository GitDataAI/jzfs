use std::collections::HashMap;
use std::path::PathBuf;
use dotenv::dotenv;

#[derive(Debug)]
pub struct AppSystemConfig {
    pub current_dir: PathBuf,
    pub env: HashMap<String, String>
}

impl AppSystemConfig {
    pub fn new() -> Self {
        dotenv().ok();
        let current_dir = std::env::current_dir().unwrap();
        let env:HashMap<String,String> = std::env::vars().collect();
        Self {
            current_dir,
            env
        }
    }
    pub fn get_env(&self, key: &str) -> Option<String> {
        self.env.get(key).map(|x|x.to_string()).or(std::env::var(key).ok())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let config = AppSystemConfig::new();
        println!("{:?}", config);
    }
}