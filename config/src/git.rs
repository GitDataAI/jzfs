use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AppGitConfig {
    #[serde(rename = "storage")]
    pub storage: Vec<AppGitStorage>,
    #[serde(rename = "default")]
    pub default: AppGitStorage,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AppGitStorage {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path")]
    pub path: PathBuf,
    #[serde(rename = "type")]
    pub storage_type: Option<GitStorageType>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum GitStorageType {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "remote")]
    Remote,
}

impl Default for AppGitConfig {
    fn default() -> Self {
        if std::fs::read_dir("./data/repo").is_err() {
            std::fs::create_dir_all("./data/repo").expect("Failed to create repo directory")
        }
        Self {
            storage: vec![],
            default: AppGitStorage {
                name: "default".to_string(),
                path: PathBuf::from("./data/repo"),
                storage_type: Some(GitStorageType::Local),
            },
        }
    }
}
