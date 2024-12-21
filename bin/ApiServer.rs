use libs::api::app_error::Error;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_MIN_STACK", "8388608");
    libs::api::init::init_api().await?;
    Ok(())
}