use libs::api::app_error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    libs::api::init::init_api().await?;
    Ok(())
}