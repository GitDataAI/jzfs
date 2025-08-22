use config::AppConfig;

pub async fn migration() {
    let config = AppConfig::init();
    let db = config.database.conn().await;
    if let Err(e) = migration::migration(&db).await {
        eprintln!("{}", e);
    }
}
