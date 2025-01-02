pub mod api;
pub mod cmd;
pub mod config;
pub mod database;
pub mod emails;
pub mod error;
pub mod git;
pub mod logic;
pub mod models;
pub mod modules;
pub mod options;
pub mod server;
pub mod utils;
#[cfg(target_os = "windows")]
pub const ROOT_PATH: &str = "E:\\test";
#[cfg(target_os = "windows")]
pub const STATIC_FILE: &str = "E:\\static";

#[cfg(not(target_os = "windows"))]
pub const ROOT_PATH: &str = "/exports";
#[cfg(not(target_os = "windows"))]
pub const STATIC_FILE: &str = "/static";

pub fn init_repo_dir() -> anyhow::Result<()> {
    let repo_dir = ROOT_PATH.to_string();
    if std::fs::read_dir(repo_dir.clone()).is_err() {
        std::fs::create_dir_all(repo_dir)?;
    }
    let static_file = STATIC_FILE.to_string();
    if std::fs::read_dir(static_file.clone()).is_err() {
        std::fs::create_dir_all(static_file)?;
    }
    Ok(())
}
