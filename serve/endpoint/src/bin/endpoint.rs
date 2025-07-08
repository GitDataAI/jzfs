use endpoint::endpoint::run;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    if let Err(e) = run().await {
        panic!("{}", e);
    }

}