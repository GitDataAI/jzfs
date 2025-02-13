use std::io;
use std::io::Error;
use std::path::PathBuf;

use crate::transport::Transport;

impl Transport {
    pub async fn text(&self, repo_path : String, file_path : String) -> io::Result<Vec<u8>> {
        let path = PathBuf::from(repo_path).join(".git");
        if !PathBuf::from(path.clone()).exists() {
            return Err(Error::new(
                io::ErrorKind::Other,
                "Repository Bare NotFount".to_string(),
            ));
        }
        let path = path.join(file_path);
        let file = tokio::fs::read(path).await?;
        Ok(file)
    }
}
