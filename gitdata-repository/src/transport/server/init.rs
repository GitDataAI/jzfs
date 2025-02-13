use crate::transport::Transport;

impl Transport {
    pub async fn server_create_repository(&self, path : String) -> anyhow::Result<()> {
        drop(git2::Repository::init(&path)?);
        Ok(())
    }
}
