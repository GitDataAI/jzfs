pub mod api;
pub mod config;
pub mod database;
pub mod emails;
pub mod error;
pub mod git;
pub mod logic;
pub mod models;
pub mod options;
pub mod server;
pub mod utils;
pub mod cmd;

#[cfg(target_os = "windows")]
pub const ROOT_PATH: &str = "E:\\test";

#[cfg(not(target_os = "windows"))]
pub const ROOT_PATH: &str = "/exports";

pub fn init_repo_dir() -> anyhow::Result<()> {
    let repo_dir = ROOT_PATH.to_string();
    if std::fs::read_dir(repo_dir.clone()).is_err() {
        std::fs::create_dir_all(repo_dir)?;
    }
    Ok(())
}
