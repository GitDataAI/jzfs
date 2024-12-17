use actix_web::{HttpRequest, HttpResponse, Responder};
use tracing::info;
use crate::ROOT_PATH;


pub fn init_git_http_backend() -> anyhow::Result<()>{
    let git_dir = ROOT_PATH.to_string();
    if std::fs::read_dir(git_dir.clone()).is_err(){
        std::fs::create_dir_all(git_dir)?;
        info!("Git dir created");
    }else { 
        info!("Git dir already exists");
    }
    Ok(())
}