use jzfs::cmd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handle = tokio::spawn(async move {
        cmd::api::api().await;
    });
    match handle.await {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!("Task failed: {}", e)),
    }
}
