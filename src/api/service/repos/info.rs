use uuid::Uuid;
use crate::api::service::repos::RepoService;

impl RepoService {
    pub async fn info(&self, _uid: Uuid) -> anyhow::Result<()>{
        Ok(())
    }
}