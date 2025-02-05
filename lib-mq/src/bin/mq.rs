use log::info;
use rdkafka::util::get_rdkafka_version;
use lib_config::AppNacos;
use lib_mq::CHANNEL;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().init();
    let nacos = AppNacos::from_env()?;
    let (version_n, version_s) = get_rdkafka_version();
    let mut naming = nacos.naming;
    naming.register(0, "mq",3).await
        .map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
    let mut consumer = lib_mq::server::server::AppKafkaConsumer::init(nacos, CHANNEL.to_string()).await?;
    consumer.topic(vec!["email".to_string()]).await?;
    consumer.listen().await;
    while let Ok(_) = tokio::signal::ctrl_c().await {
        naming
            .unregister()
            .await
            .map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::Other, e)
            })?;
        info!("Received Ctrl+C, exiting...");
        std::process::exit(0);
    }
    Ok(())
}