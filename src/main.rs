#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    jz_migration::model::migrator()
        .await
        .expect("migration error");
    tracing_subscriber::fmt().init();
    let module = jz_module::AppModule::init_env()
        .await
        .expect("init module error");
    let app = module.clone();
    tokio::spawn(async move {
        jz_ssh::SSHHandle::new(app.clone())
            .run_ssh()
            .await
            .expect("ssh error");
    });
    let service = jz_service::app::AppService::init_env().await
        .expect("init service error");
    jz_api::Api::init(module, jz_api::Settings::from_default_template(),service)
        .run()
        .await
        .expect("api error");
    
}
