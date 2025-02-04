use lib_config::AppNacos;
use lib_config::config::postgres::AppPostgresConfigKind;
use lib_entity::migration;

#[tokio::main]
async fn main() {
    let nacos = AppNacos::from_env().unwrap();
    let config = nacos.config;
    let postgres = config.postgres_connect(AppPostgresConfigKind::Write).await.unwrap();
    migration::Migrator::run(postgres).await.unwrap();
}