#![allow(non_upper_case_globals)]


#[cfg(target_os = "windows")]
pub const ROOT_PATH: &str = "E:/";

#[cfg(not(target_os = "windows"))]
pub const ROOT_PATH: &str = "/exports";

pub fn init_repo_dir() -> anyhow::Result<()> {
    let repo_dir = ROOT_PATH.to_string();
    if std::fs::read_dir(repo_dir.clone()).is_err(){
        std::fs::create_dir_all(repo_dir)?;
    }
    Ok(())
}

pub mod metadata;
pub mod api;
pub mod config;
pub mod ssh;
pub mod http;
pub mod git;
pub mod server;
pub mod rpc;
pub mod log;
pub mod template;
pub mod hash;
pub mod scheduler;