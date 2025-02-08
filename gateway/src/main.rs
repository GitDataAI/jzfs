use std::sync::Arc;
use actix_session::config::PersistentSession;
use actix_web::{web, App, HttpServer};
use actix_web::dev::{Server, ServerHandle};
use awc::cookie::time::Duration;
use awc::cookie::Key;
use lib_config::config::redis::{ClusterMaster, RedisConfigKind};
use lib_config::AppNacos;
use log::info;
use tokio::task::JoinHandle;
use lazy_static::lazy_static;
use lib_config::naming::HttpServiceNode;
use crate::api::endpoint;

mod api;
mod ping;

lazy_static! {
    static ref PORT: u16 = {
        let port = std::env::var("ALL_PORT").unwrap_or("8081".to_string());
        port.parse::<u16>().unwrap_or(8081)
    };
}

#[tokio::main]
async fn main() -> std::io::Result<()>{
    tracing_subscriber::fmt().init();
    let nacos = AppNacos::from_env()?;
    let redis = nacos.config.redis_cluster(RedisConfigKind::Session).await?;
    let mut naming = nacos.naming.clone();
    naming
        .register(PORT.clone() as i32, "gateway", 1).await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "register error"))?;
    let unregister = naming.clone();
    let (http_server_tx, mut http_server_rx) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(async move {
        let mut http_server_list = Vec::new();
        loop {
            if let Ok(mut list) = naming.clone().list_http_server(vec!["api", "web"]).await{
                list.sort();
                if http_server_list != list{
                    info!("Receive service configuration updates and start preparing the routing registration process");
                    http_server_list.clear();
                    http_server_list.extend(list.clone());
                    let mut list = list.clone();
                    for idx in list.iter_mut() {
                        let ips = idx.ips
                            .iter()
                            .map(|x|{
                                x.split(":")
                                    .collect::<Vec<&str>>()
                                    .first()
                                    .map(|x| x.to_string())
                            })
                            .filter_map(|x| x.clone())
                            .collect::<Vec<String>>();
                        let fast_ip = ping::find_fastest_dns_server(&ips.iter().map(|x| x.as_str()).collect::<Vec<&str>>(), idx.port as u16).await;
                        if let Some(fast_ip) = fast_ip{
                            idx.ips = vec![fast_ip];
                        }else {
                            idx.ips = vec![];
                        }
                    }
                    http_server_tx.send(list.clone()).ok();
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    });

    let (stop_tx, mut stop_rx) = tokio::sync::mpsc::unbounded_channel::<()>();
    tokio::spawn(async move {
        async fn run(redis: ClusterMaster, server_list: Vec<HttpServiceNode>) -> (ServerHandle, Server) {
            let serve = HttpServer::new(move || {
                App::new()
                    .wrap(actix_web::middleware::Logger::default())
                    .app_data(web::Data::new(server_list.clone()))
                    .wrap(
                        actix_session::SessionMiddleware::builder(
                            redis.clone(),
                            Key::from(&[0; 64])
                        )
                            .cookie_name("SessionID".to_string())
                            .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(30)))
                            .cookie_path("/".to_string())
                            .build()
                    )
                    .default_service(web::to(endpoint))
            })
                .max_connections(usize::MAX)
                .bind(format!("0.0.0.0:{}", PORT.clone()))
                .unwrap()
                .run();
            let handle = serve.handle();
            (handle,serve)
        }
        let mut server_list = Vec::new();

        let (handle, serve) = run(redis.clone(), server_list.clone()).await;
        let mut handle = Arc::from(handle);
        async fn run_serve(server: Server) -> JoinHandle<()> {
            tokio::spawn(async {
                server.await.unwrap();
            })
        }
        let mut serve = run_serve(serve).await;
        handle.resume().await;
        tokio::spawn(async move {
            let stop_handle = handle.clone();
            tokio::spawn(async move {
                while let Some(_) = stop_rx.recv().await {
                    stop_handle.stop(true).await;
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                    info!("Gateway Stop Success");
                }
            });
            while let Some(list) = http_server_rx.recv().await {
                info!("Detected that the configuration center API service update gateway is about to restart");
                server_list.clear();
                server_list.extend_from_slice(&list);
                info!("Gateway Stop...");
                handle.stop(true).await;
                serve.abort();
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                info!("Gateway Stop Success");
                let (new_handle, new_serve) = run(redis.clone(), server_list.clone()).await;
                handle = Arc::from(new_handle);
                serve = run_serve(new_serve).await;
                info!("Gateway Start...");
                handle.resume().await;
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                info!("Gateway Start Success");
            }
        });
    });
    while let Ok(_) = tokio::signal::ctrl_c().await {
        unregister.clone().unregister().await
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "unregister error"))?;
        stop_tx.send(()).ok();
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        info!("Receive Ctrl+c, Process while stop");
        std::process::exit(0);
    }
    Ok(())
}

