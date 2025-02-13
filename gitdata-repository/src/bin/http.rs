use lazy_static::lazy_static;
use lib_config::AppNacos;
use lib_fs::http::HttpGit;
use lib_fs::service;
use lib_fs::transport::Transport;

lazy_static! {
    static ref PORT: u16 = {
        let port = std::env::var("ALL_PORT").unwrap_or("8080".to_string());
        port.parse::<u16>().unwrap_or(8080)
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    let nacos = AppNacos::from_env()?;
    // let redis = nacos.config.redis_cluster(RedisConfigKind::Session).await?;
    let mut naming = nacos.naming.clone();
    let state = service::AppFsState::init(nacos.clone())
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "init error"))?;
    naming
        .register(PORT.clone() as i32, "api", 1)
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "register error"))?;
    // let mq = AppKafkaClient::init(nacos).await?;
    let rt = tokio::spawn(async move {
        HttpGit::new(state, Transport {}, PORT.clone())
            .await
            .expect("http git error");
    });
    while let Some(_) = tokio::signal::ctrl_c().await.ok() {
        rt.abort();
        naming
            .unregister()
            .await
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "unregister error"))?;
        break;
    }

    Ok(())
}
