use std::io;
use uuid::Uuid;
use crate::http::GIT_ROOT;
use crate::services::AppState;

impl AppState {
    pub async fn product_data_download_zip(&self, product: Uuid) -> io::Result<actix_files::NamedFile> {
        let product = self.product_info(product).await?;
        let repo = self.repo_get_by_uid(product.repository_uid).await?;
        let hash = product.hash;
        let path = format!("{}/{}/{}/", GIT_ROOT, repo.node_uid, repo.uid);
        let zip_path = format!("{}/product/{}.zip", path, hash);
        if std::fs::metadata(&zip_path).is_err() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "not found"));
        }
        let file = actix_files::NamedFile::open(zip_path)?;
        Ok(file)
    }
}