use libs::http::GitHttp;

#[tokio::main(worker_threads = 16)]
async fn main() -> anyhow::Result<()>{
    GitHttp().await?;
    Ok(())
}