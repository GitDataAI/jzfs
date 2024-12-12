use std::env::consts::OS;
use std::time::Instant;
use actix_web::{web, Responder};

#[utoipa::path(
    get,
    path = "/api/version",
    description = "获取服务启动时间，软件版本，运行时操作系统，进程ID",
    responses(
        (status = 200, description = "获取成功", body = String)
    )
)]
pub async fn api_version(time: web::Data<Instant>) -> impl Responder{
    format!("服务启动时长: {:?}\n版本: {}\n操作系统: {}\n进程ID: {}", time.elapsed(), "0.1", OS.to_string(), std::process::id())
}